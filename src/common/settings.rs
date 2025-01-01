use bevy::prelude::{Component, Resource};
use serde::{Deserialize, Serialize};

// 设置文件
#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct Settings {
    pub service: String,
    pub port: String,
    pub ext_dir: String,
}

#[derive(Component)]
pub struct ServiceNameInput;

#[derive(Component)]
pub struct PortNameInput;
