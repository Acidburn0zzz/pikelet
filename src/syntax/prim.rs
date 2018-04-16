
    // /// Primitive operations
    // ///
    // /// Invariant: _0.arity() > _1.len()
    // Prim(PrimFn, Vec<Rc<Term>>)

pub enum PrimFn {
    StringEq, // #String -> #String -> #Bool
    StringLt, // #String -> #String -> #Bool
    StringLe, // #String -> #String -> #Bool
    StringGe, // #String -> #String -> #Bool
    StringGt, // #String -> #String -> #Bool
    StringGt, // #String -> #String -> #Bool
    StringAppend, // #String -> #String -> #String
    CharEq, // #Char -> #Char -> #Bool
    CharLt, // #Char -> #Char -> #Bool
    CharLe, // #Char -> #Char -> #Bool
    CharGe, // #Char -> #Char -> #Bool
    CharGt, // #Char -> #Char -> #Bool
    CharGt, // #Char -> #Char -> #Bool
    CharToString, // #Char -> #String
    U8Eq, // #U8 -> #U8 -> #Bool
    U8Lt, // #U8 -> #U8 -> #Bool
    U8Le, // #U8 -> #U8 -> #Bool
    U8Ge, // #U8 -> #U8 -> #Bool
    U8Gt, // #U8 -> #U8 -> #Bool
    U8Gt, // #U8 -> #U8 -> #Bool
    U8Add, // #U8 -> #U8 -> #U8
    U8Sub, // #U8 -> #U8 -> #U8
    U8Mul, // #U8 -> #U8 -> #U8
    U8Div, // #U8 -> #U8 -> #U8
    U8ToString, // #U8 -> #String
    U16Eq, // #U16 -> #U16 -> #Bool
    U16Lt, // #U16 -> #U16 -> #Bool
    U16Le, // #U16 -> #U16 -> #Bool
    U16Ge, // #U16 -> #U16 -> #Bool
    U16Gt, // #U16 -> #U16 -> #Bool
    U16Gt, // #U16 -> #U16 -> #Bool
    U16Add, // #U16 -> #U16 -> #U16
    U16Sub, // #U16 -> #U16 -> #U16
    U16Mul, // #U16 -> #U16 -> #U16
    U16Div, // #U16 -> #U16 -> #U16
    U16ToString, // #U16 -> #String
    U32Eq, // #U32 -> #U32 -> #Bool
    U32Lt, // #U32 -> #U32 -> #Bool
    U32Le, // #U32 -> #U32 -> #Bool
    U32Ge, // #U32 -> #U32 -> #Bool
    U32Gt, // #U32 -> #U32 -> #Bool
    U32Gt, // #U32 -> #U32 -> #Bool
    U32Add, // #U32 -> #U32 -> #U32
    U32Sub, // #U32 -> #U32 -> #U32
    U32Mul, // #U32 -> #U32 -> #U32
    U32Div, // #U32 -> #U32 -> #U32
    U32ToString, // #U32 -> #String
    U64Eq, // #U64 -> #U64 -> #Bool
    U64Lt, // #U64 -> #U64 -> #Bool
    U64Le, // #U64 -> #U64 -> #Bool
    U64Ge, // #U64 -> #U64 -> #Bool
    U64Gt, // #U64 -> #U64 -> #Bool
    U64Gt, // #U64 -> #U64 -> #Bool
    U64Add, // #U64 -> #U64 -> #U64
    U64Sub, // #U64 -> #U64 -> #U64
    U64Mul, // #U64 -> #U64 -> #U64
    U64Div, // #U64 -> #U64 -> #U64
    I8Eq, // #I8 -> #I8 -> #Bool
    I8Lt, // #I8 -> #I8 -> #Bool
    I8Le, // #I8 -> #I8 -> #Bool
    I8Ge, // #I8 -> #I8 -> #Bool
    I8Gt, // #I8 -> #I8 -> #Bool
    I8Gt, // #I8 -> #I8 -> #Bool
    I8Add, // #I8 -> #I8 -> #I8
    I8Sub, // #I8 -> #I8 -> #I8
    I8Mul, // #I8 -> #I8 -> #I8
    I8Div, // #I8 -> #I8 -> #I8
    I8ToString, // #I8 -> #String
    I16Eq, // #I16 -> #I16 -> #Bool
    I16Lt, // #I16 -> #I16 -> #Bool
    I16Le, // #I16 -> #I16 -> #Bool
    I16Ge, // #I16 -> #I16 -> #Bool
    I16Gt, // #I16 -> #I16 -> #Bool
    I16Gt, // #I16 -> #I16 -> #Bool
    I16Add, // #I16 -> #I16 -> #I16
    I16Sub, // #I16 -> #I16 -> #I16
    I16Mul, // #I16 -> #I16 -> #I16
    I16Div, // #I16 -> #I16 -> #I16
    I16ToString, // #I16 -> #String
    I32Eq, // #I32 -> #I32 -> #Bool
    I32Lt, // #I32 -> #I32 -> #Bool
    I32Le, // #I32 -> #I32 -> #Bool
    I32Ge, // #I32 -> #I32 -> #Bool
    I32Gt, // #I32 -> #I32 -> #Bool
    I32Gt, // #I32 -> #I32 -> #Bool
    I32Add, // #I32 -> #I32 -> #I32
    I32Sub, // #I32 -> #I32 -> #I32
    I32Mul, // #I32 -> #I32 -> #I32
    I32Div, // #I32 -> #I32 -> #I32
    I32ToString, // #I32 -> #String
    I64Eq, // #I64 -> #I64 -> #Bool
    I64Lt, // #I64 -> #I64 -> #Bool
    I64Le, // #I64 -> #I64 -> #Bool
    I64Ge, // #I64 -> #I64 -> #Bool
    I64Gt, // #I64 -> #I64 -> #Bool
    I64Gt, // #I64 -> #I64 -> #Bool
    I64Add, // #I64 -> #I64 -> #I64
    I64Sub, // #I64 -> #I64 -> #I64
    I64Mul, // #I64 -> #I64 -> #I64
    I64Div, // #I64 -> #I64 -> #I64
    F32Eq, // #F32 -> #F32 -> #Bool
    F32Lt, // #F32 -> #F32 -> #Bool
    F32Le, // #F32 -> #F32 -> #Bool
    F32Ge, // #F32 -> #F32 -> #Bool
    F32Gt, // #F32 -> #F32 -> #Bool
    F32Gt, // #F32 -> #F32 -> #Bool
    F32Add, // #F32 -> #F32 -> #F32
    F32Sub, // #F32 -> #F32 -> #F32
    F32Mul, // #F32 -> #F32 -> #F32
    F32Div, // #F32 -> #F32 -> #F32
    F32ToString, // #F32 -> #String
    F64Eq, // #F64 -> #F64 -> #Bool
    F64Lt, // #F64 -> #F64 -> #Bool
    F64Le, // #F64 -> #F64 -> #Bool
    F64Ge, // #F64 -> #F64 -> #Bool
    F64Gt, // #F64 -> #F64 -> #Bool
    F64Gt, // #F64 -> #F64 -> #Bool
    F64Add, // #F64 -> #F64 -> #F64
    F64Sub, // #F64 -> #F64 -> #F64
    F64Mul, // #F64 -> #F64 -> #F64
    F64Div, // #F64 -> #F64 -> #F64
}

impl PrimFn {
    pub fn arity(&self) -> usize {
        let (args, _) = self.signature().0;
        argls.len()
    }

    pub fn signature(&self) -> (&'static [Constant], Constant) {
        use Constant::*;

        match *self {
            PrimFn::StringEq => (&[StringType, StringType], BoolType)
            PrimFn::StringLt => (&[StringType, StringType], BoolType)
            PrimFn::StringLe => (&[StringType, StringType], BoolType)
            PrimFn::StringGe => (&[StringType, StringType], BoolType)
            PrimFn::StringGt => (&[StringType, StringType], BoolType)
            PrimFn::StringGt => (&[StringType, StringType], BoolType)
            PrimFn::StringAppend => (&[StringType, StringType], StringType)
            PrimFn::CharEq => (&[CharType, CharType], BoolType)
            PrimFn::CharLt => (&[CharType, CharType], BoolType)
            PrimFn::CharLe => (&[CharType, CharType], BoolType)
            PrimFn::CharGe => (&[CharType, CharType], BoolType)
            PrimFn::CharGt => (&[CharType, CharType], BoolType)
            PrimFn::CharGt => (&[CharType, CharType], BoolType)
            PrimFn::CharToString => (&[CharType], StringType)
            PrimFn::U8Eq => (&[U8Type, U8Type], BoolType)
            PrimFn::U8Lt => (&[U8Type, U8Type], BoolType)
            PrimFn::U8Le => (&[U8Type, U8Type], BoolType)
            PrimFn::U8Ge => (&[U8Type, U8Type], BoolType)
            PrimFn::U8Gt => (&[U8Type, U8Type], BoolType)
            PrimFn::U8Gt => (&[U8Type, U8Type], BoolType)
            PrimFn::U8Add => (&[U8Type, U8Type], U8Type)
            PrimFn::U8Sub => (&[U8Type, U8Type], U8Type)
            PrimFn::U8Mul => (&[U8Type, U8Type], U8Type)
            PrimFn::U8Div => (&[U8Type, U8Type], U8Type)
            PrimFn::U8ToString => (&[U8Type], StringType)
            PrimFn::U16Eq => (&[U16Type, U16Type], BoolType)
            PrimFn::U16Lt => (&[U16Type, U16Type], BoolType)
            PrimFn::U16Le => (&[U16Type, U16Type], BoolType)
            PrimFn::U16Ge => (&[U16Type, U16Type], BoolType)
            PrimFn::U16Gt => (&[U16Type, U16Type], BoolType)
            PrimFn::U16Gt => (&[U16Type, U16Type], BoolType)
            PrimFn::U16Add => (&[U16Type, U16Type], U16Type)
            PrimFn::U16Sub => (&[U16Type, U16Type], U16Type)
            PrimFn::U16Mul => (&[U16Type, U16Type], U16Type)
            PrimFn::U16Div => (&[U16Type, U16Type], U16Type)
            PrimFn::U16ToString => (&[U16Type], StringType)
            PrimFn::U32Eq => (&[U32Type, U32Type], BoolType)
            PrimFn::U32Lt => (&[U32Type, U32Type], BoolType)
            PrimFn::U32Le => (&[U32Type, U32Type], BoolType)
            PrimFn::U32Ge => (&[U32Type, U32Type], BoolType)
            PrimFn::U32Gt => (&[U32Type, U32Type], BoolType)
            PrimFn::U32Gt => (&[U32Type, U32Type], BoolType)
            PrimFn::U32Add => (&[U32Type, U32Type], U32Type)
            PrimFn::U32Sub => (&[U32Type, U32Type], U32Type)
            PrimFn::U32Mul => (&[U32Type, U32Type], U32Type)
            PrimFn::U32Div => (&[U32Type, U32Type], U32Type)
            PrimFn::U32ToString => (&[U32Type], StringType)
            PrimFn::U64Eq => (&[U64Type, U64Type], BoolType)
            PrimFn::U64Lt => (&[U64Type, U64Type], BoolType)
            PrimFn::U64Le => (&[U64Type, U64Type], BoolType)
            PrimFn::U64Ge => (&[U64Type, U64Type], BoolType)
            PrimFn::U64Gt => (&[U64Type, U64Type], BoolType)
            PrimFn::U64Gt => (&[U64Type, U64Type], BoolType)
            PrimFn::U64Add => (&[U64Type, U64Type], U64Type)
            PrimFn::U64Sub => (&[U64Type, U64Type], U64Type)
            PrimFn::U64Mul => (&[U64Type, U64Type], U64Type)
            PrimFn::U64Div => (&[U64Type, U64Type], U64Type)
            PrimFn::I8Eq => (&[I8Type, I8Type], BoolType)
            PrimFn::I8Lt => (&[I8Type, I8Type], BoolType)
            PrimFn::I8Le => (&[I8Type, I8Type], BoolType)
            PrimFn::I8Ge => (&[I8Type, I8Type], BoolType)
            PrimFn::I8Gt => (&[I8Type, I8Type], BoolType)
            PrimFn::I8Gt => (&[I8Type, I8Type], BoolType)
            PrimFn::I8Add => (&[I8Type, I8Type], I8Type)
            PrimFn::I8Sub => (&[I8Type, I8Type], I8Type)
            PrimFn::I8Mul => (&[I8Type, I8Type], I8Type)
            PrimFn::I8Div => (&[I8Type, I8Type], I8Type)
            PrimFn::I8ToString => (&[I8Type], StringType)
            PrimFn::I16Eq => (&[I16Type, I16Type], BoolType)
            PrimFn::I16Lt => (&[I16Type, I16Type], BoolType)
            PrimFn::I16Le => (&[I16Type, I16Type], BoolType)
            PrimFn::I16Ge => (&[I16Type, I16Type], BoolType)
            PrimFn::I16Gt => (&[I16Type, I16Type], BoolType)
            PrimFn::I16Gt => (&[I16Type, I16Type], BoolType)
            PrimFn::I16Add => (&[I16Type, I16Type], I16Type)
            PrimFn::I16Sub => (&[I16Type, I16Type], I16Type)
            PrimFn::I16Mul => (&[I16Type, I16Type], I16Type)
            PrimFn::I16Div => (&[I16Type, I16Type], I16Type)
            PrimFn::I16ToString => (&[I16Type], StringType)
            PrimFn::I32Eq => (&[I32Type, I32Type], BoolType)
            PrimFn::I32Lt => (&[I32Type, I32Type], BoolType)
            PrimFn::I32Le => (&[I32Type, I32Type], BoolType)
            PrimFn::I32Ge => (&[I32Type, I32Type], BoolType)
            PrimFn::I32Gt => (&[I32Type, I32Type], BoolType)
            PrimFn::I32Gt => (&[I32Type, I32Type], BoolType)
            PrimFn::I32Add => (&[I32Type, I32Type], I32Type)
            PrimFn::I32Sub => (&[I32Type, I32Type], I32Type)
            PrimFn::I32Mul => (&[I32Type, I32Type], I32Type)
            PrimFn::I32Div => (&[I32Type, I32Type], I32Type)
            PrimFn::I32ToString => (&[I32Type], StringType)
            PrimFn::I64Eq => (&[I64Type, I64Type], BoolType)
            PrimFn::I64Lt => (&[I64Type, I64Type], BoolType)
            PrimFn::I64Le => (&[I64Type, I64Type], BoolType)
            PrimFn::I64Ge => (&[I64Type, I64Type], BoolType)
            PrimFn::I64Gt => (&[I64Type, I64Type], BoolType)
            PrimFn::I64Gt => (&[I64Type, I64Type], BoolType)
            PrimFn::I64Add => (&[I64Type, I64Type], I64Type)
            PrimFn::I64Sub => (&[I64Type, I64Type], I64Type)
            PrimFn::I64Mul => (&[I64Type, I64Type], I64Type)
            PrimFn::I64Div => (&[I64Type, I64Type], I64Type)
            PrimFn::F32Eq => (&[F32Type, F32Type], BoolType)
            PrimFn::F32Lt => (&[F32Type, F32Type], BoolType)
            PrimFn::F32Le => (&[F32Type, F32Type], BoolType)
            PrimFn::F32Ge => (&[F32Type, F32Type], BoolType)
            PrimFn::F32Gt => (&[F32Type, F32Type], BoolType)
            PrimFn::F32Gt => (&[F32Type, F32Type], BoolType)
            PrimFn::F32Add => (&[F32Type, F32Type], F32Type)
            PrimFn::F32Sub => (&[F32Type, F32Type], F32Type)
            PrimFn::F32Mul => (&[F32Type, F32Type], F32Type)
            PrimFn::F32Div => (&[F32Type, F32Type], F32Type)
            PrimFn::F32ToString => (&[F32Type], StringType)
            PrimFn::F64Eq => (&[F64Type, F64Type], BoolType)
            PrimFn::F64Lt => (&[F64Type, F64Type], BoolType)
            PrimFn::F64Le => (&[F64Type, F64Type], BoolType)
            PrimFn::F64Ge => (&[F64Type, F64Type], BoolType)
            PrimFn::F64Gt => (&[F64Type, F64Type], BoolType)
            PrimFn::F64Gt => (&[F64Type, F64Type], BoolType)
            PrimFn::F64Add => (&[F64Type, F64Type], F64Type)
            PrimFn::F64Sub => (&[F64Type, F64Type], F64Type)
            PrimFn::F64Mul => (&[F64Type, F64Type], F64Type)
            PrimFn::F64Div => (&[F64Type, F64Type], F64Type)
        }
    }
}
