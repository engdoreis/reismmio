// Copyright (c) 2025 Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
use svd_rs::access as svd_rs;
#[derive(Debug, Clone, Default, strum::Display, strum::IntoStaticStr)]
pub enum Permissions {
    #[default]
    ReadWrite,
    ReadWriteOnce,
    Read,
    Write,
    WriteOnce,
}

impl From<&str> for Permissions {
    fn from(s: &str) -> Self {
        match s {
            "read-only" => Self::Read,
            "write-only" => Self::Write,
            "read-write" => Self::ReadWrite,
            _ => panic!("{} unsuported", s),
        }
    }
}

impl From<svd_rs::Access> for Permissions {
    fn from(s: svd_rs::Access) -> Self {
        match s {
            svd_rs::Access::ReadOnly => Permissions::Read,
            svd_rs::Access::ReadWrite => Permissions::ReadWrite,
            svd_rs::Access::ReadWriteOnce => Permissions::ReadWriteOnce,
            svd_rs::Access::WriteOnce => Permissions::WriteOnce,
            svd_rs::Access::WriteOnly => Permissions::Write,
        }
    }
}
