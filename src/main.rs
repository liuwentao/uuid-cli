use clap::{Parser, ValueEnum};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 可选的 UUID 版本（如果未提供，则默认使用 v4 且会移除连字符）
    #[arg(value_enum)]
    kind: Option<UuidVersion>,
    #[arg(short = 'n', long = "count", default_value_t = 1)]
    count: usize,
}

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lowercase")]
enum UuidVersion {
    V4,
    V7,
}

fn main() {
    // 1. 解析命令行参数
    let args = Args::parse();

    // 2. 根据用户选择的版本生成指定数量的 UUID 并打印
    // 如果用户显式传入了版本（例如 `v4` 或 `v7`），则保留原始带连字符的格式
    // 如果未传入版本，则默认使用 v4 并移除连字符
    let (chosen_version, strip_hyphens) = match args.kind {
        Some(k) => (k, false),
        None => (UuidVersion::V4, true),
    };

    for _ in 0..args.count {
        let new_uuid = match chosen_version {
            UuidVersion::V4 => Uuid::new_v4(),
            UuidVersion::V7 => Uuid::now_v7(),
        };

        if strip_hyphens {
            println!("{}", new_uuid.to_string().replace("-", ""));
        } else {
            println!("{}", new_uuid);
        }
    }
}