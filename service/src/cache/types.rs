#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Types {
    SystemAuthJwt,
    SystemAuthLoginCaptcha,
    SystemAuthLoginMobile,
    SystemAuthLoginQrCode,
    MemberAuthRegisterEmail,
    MemberAuthLoginEmail,
}
impl From<i32> for Types {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::SystemAuthJwt,
            1 => Self::SystemAuthLoginCaptcha,
            2 => Self::SystemAuthLoginMobile,
            3 => Self::MemberAuthRegisterEmail,
            4 => Self::SystemAuthLoginQrCode,
            5 => Self::MemberAuthLoginEmail,
            _ => Self::SystemAuthJwt,
        }
    }
}
impl From<Types> for i32 {
    fn from(value: Types) -> Self {
        match value {
            Types::SystemAuthJwt => 0,
            Types::SystemAuthLoginCaptcha => 1,
            Types::SystemAuthLoginMobile => 2,
            Types::MemberAuthRegisterEmail => 3,
            Types::SystemAuthLoginQrCode => 4,
            Types::MemberAuthLoginEmail => 5,
        }
    }
}
