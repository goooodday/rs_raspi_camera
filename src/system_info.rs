use std::env;

/// System information structure
#[derive(Clone)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub platform_name: String,
    pub platform_icon: String,
    pub target: String,
    pub is_cross_compiled: bool,
}

impl SystemInfo {
    pub fn detect() -> Self {
        let target = env::var("CARGO_CFG_TARGET_ARCH")
            .unwrap_or_else(|_| std::env::consts::ARCH.to_string());

        let os =
            env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| std::env::consts::OS.to_string());

        let full_target = env::var("TARGET").unwrap_or_else(|_| {
            format!(
                "{}-unknown-{}-gnu",
                std::env::consts::ARCH,
                std::env::consts::OS
            )
        });

        // Detect if we're cross-compiled
        let is_cross_compiled = target != std::env::consts::ARCH || os != std::env::consts::OS;

        // Determine platform information based on target architecture and OS
        let (platform_name, platform_icon, arch_display) = match (target.as_str(), os.as_str()) {
            ("aarch64", "linux") => ("라즈베리파이", "🍇", "ARM64".to_string()),
            ("arm", "linux") => ("라즈베리파이", "🍇", "ARM32".to_string()),
            ("x86_64", "linux") => ("Linux PC", "🐧", "x86_64".to_string()),
            ("x86_64", "macos") => ("Mac", "🍎", "Intel/Apple".to_string()),
            ("aarch64", "macos") => ("Mac", "🍎", "Apple Silicon".to_string()),
            ("x86_64", "windows") => ("Windows PC", "🪟", "x86_64".to_string()),
            ("i686", "windows") => ("Windows PC", "🪟", "x86".to_string()),
            _ => ("Unknown Platform", "💻", target.clone()),
        };

        SystemInfo {
            os,
            arch: arch_display.to_string(),
            platform_name: platform_name.to_string(),
            platform_icon: platform_icon.to_string(),
            target: full_target,
            is_cross_compiled,
        }
    }

    pub fn get_compilation_info(&self) -> String {
        if self.is_cross_compiled {
            "크로스컴파일됨".to_string()
        } else {
            "네이티브 컴파일".to_string()
        }
    }

    pub fn get_network_info(&self) -> String {
        match self.platform_name.as_str() {
            "라즈베리파이" => "0.0.0.0:3000 (외부접근 가능)".to_string(),
            _ => "127.0.0.1:3000 (로컬)".to_string(),
        }
    }

    pub fn get_runtime_environment(&self) -> String {
        if self.is_cross_compiled {
            format!("타겟: {}", self.target)
        } else {
            format!("네이티브: {}", self.target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_detection() {
        let system_info = SystemInfo::detect();
        assert!(!system_info.os.is_empty());
        assert!(!system_info.arch.is_empty());
        assert!(!system_info.platform_name.is_empty());
        assert!(!system_info.platform_icon.is_empty());
    }

    #[test]
    fn test_compilation_info() {
        let system_info = SystemInfo::detect();
        let compilation_info = system_info.get_compilation_info();
        assert!(!compilation_info.is_empty());
    }

    #[test]
    fn test_network_info() {
        let system_info = SystemInfo::detect();
        let network_info = system_info.get_network_info();
        assert!(!network_info.is_empty());
    }
}
