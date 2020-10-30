use serde::export::Formatter;

pub enum PermissionLvl {
    ADMIN,
    UNAUTHORIZED,
}

impl std::cmp::PartialEq for PermissionLvl {
    fn eq(&self, other: &Self) -> bool {
        if std::mem::discriminant(self) == std::mem::discriminant(other) {
            return true;
        }
        return false;
    }
}

impl std::fmt::Display for PermissionLvl {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            PermissionLvl::ADMIN => write!(f, "ADMIN"),
            PermissionLvl::UNAUTHORIZED => write!(f, "UNAUTHORIZED")
        }
    }
}


pub enum S3DownloadError {
    NotFound,
    AccessDenied,
    Other(String)
}