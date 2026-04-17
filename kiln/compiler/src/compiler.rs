use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::types::BasicTypeEnum;
use inkwell::types::FunctionType;
use inkwell::passes::PassManager;
use std::collections::HashMap;

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    // maps variable names to their stack allocations
    pub named_values: HashMap<String, PointerValue<'ctx>>,
     // tracks function prototypes so we can re-declare them in new modules
    pub function_protos: HashMap<String, crate::ast::FuncDef>,
    // per-function optimizer
    pub fpm: PassManager<FunctionValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        Compiler {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
            named_values: HashMap::new(),

            // set up the legacy function pass manager
        let fpm = PassManager::create(&module);
        fpm.add_instruction_combining_pass();  // peephole cleanup
        fpm.add_reassociate_pass();            // reassociate expressions
        fpm.add_gvn_pass();                    // eliminate common subexpressions
        fpm.add_cfg_simplification_pass();     // clean up control flow
        fpm.initialize();

        Compiler {
            context,
            module,
            builder,
            named_values: HashMap::new(),
            function_protos: HashMap::new(),
            fpm,
        }
        }
    }

impl<'ctx> Compiler<'ctx> {
    // look up a function by name, re-declaring its prototype if needed
    pub fn get_function(&mut self, name: &str) -> Option<FunctionValue<'ctx>> {
        // already in the current module?
        if let Some(f) = self.module.get_function(name) {
            return Some(f);
        }

        // do we have a saved prototype we can re-declare?
        if let Some(proto) = self.function_protos.get(name).cloned() {
            // declare just the signature, no body
            let param_types: Vec<inkwell::types::BasicMetadataTypeEnum> = proto.params
                .iter()
                .map(|p| self.resolve_type(&p.ty).into())
                .collect();

            let fn_type = match &proto.return_type {
                Some(ty) => self.resolve_type(ty).fn_type(&param_types, false),
                None => self.context.i64_type().fn_type(&param_types, false),
            };

            let f = self.module.add_function(&proto.name, fn_type, None);
            return Some(f);
        }

        None
    }
}

impl<'ctx> Compiler<'ctx> {
    pub fn resolve_type(&self, ty: &crate::ast::Type) -> BasicTypeEnum<'ctx> {
        match ty {
            crate::ast::Type::Int    => self.context.i64_type().into(),
            crate::ast::Type::Float  => self.context.f64_type().into(),
            crate::ast::Type::Bool   => self.context.bool_type().into(),
            crate::ast::Type::Byte   => self.context.i8_type().into(),
            crate::ast::Type::Ptr    => self.context
                                            .i8_type()
                                            .ptr_type(inkwell::AddressSpace::default())
                                            .into(),
            crate::ast::Type::StringType => self.context
                                                .i8_type()
                                                .ptr_type(inkwell::AddressSpace::default())
                                                .into(),
            crate::ast::Type::Array(inner, size) => {
                let inner_ty = self.resolve_type(inner);
                inner_ty.array_type(*size as u32).into()
            }
            crate::ast::Type::Array2D(inner, rows, cols) => {
                let inner_ty = self.resolve_type(inner);
                inner_ty.array_type(*cols as u32)
                        .array_type(*rows as u32)
                        .into()
            }
        }
    }
}


impl<'ctx> Compiler<'ctx> {
    pub fn codegen_expr(&mut self, expr: &crate::ast::Expr) 
        -> Result<BasicValueEnum<'ctx>, String> 
    {
        match expr {
            // Integer literal
            crate::ast::Expr::IntLit(n) => {
                Ok(self.context
                    .i64_type()
                    .const_int(*n as u64, true)
                    .into())
            }

            // Float literal
            crate::ast::Expr::FloatLit(f) => {
                Ok(self.context
                    .f64_type()
                    .const_float(*f)
                    .into())
            }

            // Bool literal
            crate::ast::Expr::BoolLit(b) => {
                Ok(self.context
                    .bool_type()
                    .const_int(if *b { 1 } else { 0 }, false)
                    .into())
            }

            // Variable lookup
            crate::ast::Expr::Identifier(name) => {
                let ptr = self.named_values.get(name)
                    .ok_or_else(|| format!("Unknown variable: {}", name))?;
                // load the value from the stack slot
                Ok(self.builder.build_load(
                    self.context.i64_type(), // TODO: track types properly
                    *ptr,
                    name
                ).unwrap().into())
            }

            // Binary operations
            crate::ast::Expr::BinaryOp { op, lhs, rhs } => {
                let l = self.codegen_expr(lhs)?;
                let r = self.codegen_expr(rhs)?;
                self.codegen_binop(op, l, r)
            }

            // Function call
            crate::ast::Expr::Call { callee, args } => {
                let func = self.module.get_function(callee)
                    .ok_or_else(|| format!("Unknown function: {}", callee))?;

                let mut arg_vals = Vec::new();
                for arg in args {
                    let val = self.codegen_expr(arg)?;
                    arg_vals.push(val.into());
                }

                let call = self.builder
                    .build_call(func, &arg_vals, "calltmp")
                    .unwrap();

                call.try_as_basic_value()
                    .left()
                    .ok_or("Function returned void".to_string())
            }

            // Field access e.g. console.print — resolved at a higher level
            // for now we just error; module dispatch happens before codegen
            crate::ast::Expr::FieldAccess { .. } => {
                Err("Field access should be resolved before codegen".to_string())
            }

            _ => Err("Unsupported expression in codegen".to_string()),
        }
    }

    fn codegen_binop(
        &mut self,
        op: &crate::ast::BinOp,
        l: BasicValueEnum<'ctx>,
        r: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Integer arithmetic path
        if l.is_int_value() && r.is_int_value() {
            let lv = l.into_int_value();
            let rv = r.into_int_value();
            let result = match op {
                crate::ast::BinOp::Add   => self.builder.build_int_add(lv, rv, "addtmp"),
                crate::ast::BinOp::Sub   => self.builder.build_int_sub(lv, rv, "subtmp"),
                crate::ast::BinOp::Mul   => self.builder.build_int_mul(lv, rv, "multmp"),
                crate::ast::BinOp::Div   => self.builder.build_int_signed_div(lv, rv, "divtmp"),
                crate::ast::BinOp::Mod   => self.builder.build_int_signed_rem(lv, rv, "modtmp"),
                crate::ast::BinOp::Eq    => self.builder.build_int_compare(
                                                inkwell::IntPredicate::EQ, lv, rv, "eqtmp"),
                crate::ast::BinOp::NotEq => self.builder.build_int_compare(
                                                inkwell::IntPredicate::NE, lv, rv, "netmp"),
                crate::ast::BinOp::Lt    => self.builder.build_int_compare(
                                                inkwell::IntPredicate::SLT, lv, rv, "lttmp"),
                crate::ast::BinOp::Gt    => self.builder.build_int_compare(
                                                inkwell::IntPredicate::SGT, lv, rv, "gttmp"),
                crate::ast::BinOp::LtEq  => self.builder.build_int_compare(
                                                inkwell::IntPredicate::SLE, lv, rv, "letmp"),
                crate::ast::BinOp::GtEq  => self.builder.build_int_compare(
                                                inkwell::IntPredicate::SGE, lv, rv, "getmp"),
                crate::ast::BinOp::And   => self.builder.build_and(lv, rv, "andtmp"),
                crate::ast::BinOp::Or    => self.builder.build_or(lv, rv, "ortmp"),
            };
            return Ok(result.unwrap().into());
        }

        // Float arithmetic path
        if l.is_float_value() && r.is_float_value() {
            let lv = l.into_float_value();
            let rv = r.into_float_value();
            let result = match op {
                crate::ast::BinOp::Add   => self.builder.build_float_add(lv, rv, "addtmp"),
                crate::ast::BinOp::Sub   => self.builder.build_float_sub(lv, rv, "subtmp"),
                crate::ast::BinOp::Mul   => self.builder.build_float_mul(lv, rv, "multmp"),
                crate::ast::BinOp::Div   => self.builder.build_float_div(lv, rv, "divtmp"),
                _ => return Err("Unsupported float operator".to_string()),
            };
            return Ok(result.unwrap().into());
        }

        Err("Type mismatch in binary operation".to_string())
    }
}

impl<'ctx> Compiler<'ctx> {
    pub fn codegen_func(&mut self, func: &crate::ast::FuncDef)
        -> Result<FunctionValue<'ctx>, String>
    {
        // save prototype so other modules can re-declare it
        self.function_protos.insert(func.name.clone(), func.clone());

        let param_types: Vec<inkwell::types::BasicMetadataTypeEnum> = func.params
            .iter()
            .map(|p| self.resolve_type(&p.ty).into())
            .collect();

        let fn_type = match &func.return_type {
            Some(ty) => {
                let ret = self.resolve_type(ty);
                ret.fn_type(&param_types, false)
            }
            None => self.context.i64_type().fn_type(&param_types, false),
        };

        let llvm_func = self.module.add_function(&func.name, fn_type, None);

        let entry = self.context.append_basic_block(llvm_func, "entry");
        self.builder.position_at_end(entry);

        self.named_values.clear();
        for (i, param) in func.params.iter().enumerate() {
            let llvm_param = llvm_func.get_nth_param(i as u32).unwrap();
            let alloca = self.builder
                .build_alloca(self.resolve_type(&param.ty), &param.name)
                .unwrap();
            self.builder.build_store(alloca, llvm_param).unwrap();
            self.named_values.insert(param.name.clone(), alloca);
        }

        for stmt in &func.body {
            self.codegen_stmt(stmt)?;
        }

        // run the optimizer on the finished function
        self.fpm.run_on(&llvm_func);

        Ok(llvm_func)
    }
}

impl<'ctx> Compiler<'ctx> {
    pub fn codegen_stmt(&mut self, stmt: &crate::ast::Stmt) 
        -> Result<(), String> 
    {
        match stmt {
            crate::ast::Stmt::Let { name, ty, value } => {
                let llvm_ty = match ty {
                    Some(t) => self.resolve_type(t),
                    None => self.context.i64_type().into(), // infer later
                };
                let alloca = self.builder.build_alloca(llvm_ty, name).unwrap();
                if let Some(val_expr) = value {
                    let val = self.codegen_expr(val_expr)?;
                    self.builder.build_store(alloca, val).unwrap();
                }
                self.named_values.insert(name.clone(), alloca);
                Ok(())
            }

            crate::ast::Stmt::Assign { target, value } => {
                let val = self.codegen_expr(value)?;
                let ptr = self.named_values.get(target)
                    .ok_or_else(|| format!("Unknown variable: {}", target))?;
                self.builder.build_store(*ptr, val).unwrap();
                Ok(())
            }

            crate::ast::Stmt::Return(expr) => {
                match expr {
                    Some(e) => {
                        let val = self.codegen_expr(e)?;
                        self.builder.build_return(Some(&val)).unwrap();
                    }
                    None => {
                        self.builder.build_return(None).unwrap();
                    }
                }
                Ok(())
            }

            crate::ast::Stmt::ExprStmt(expr) => {
                self.codegen_expr(expr)?;
                Ok(())
            }

            // If/else if/else
            crate::ast::Stmt::If { condition, body, else_ifs, else_body } => {
                self.codegen_if(condition, body, else_ifs, else_body)
            }

            // While loop
            crate::ast::Stmt::While { condition, body } => {
                self.codegen_while(condition, body)
            }

            // Infinite loop
            crate::ast::Stmt::Loop { body } => {
                self.codegen_loop(body)
            }

            crate::ast::Stmt::Break => {
                // break target block must be tracked — placeholder for now
                Err("break requires loop context tracking (coming next)".to_string())
            }

            crate::ast::Stmt::Continue => {
                Err("continue requires loop context tracking (coming next)".to_string())
            }

            _ => Err("Unsupported statement in codegen".to_string()),
        }
    }
}
