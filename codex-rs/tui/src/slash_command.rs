use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Approvals,
    Permissions,
    #[strum(serialize = "setup-default-sandbox")]
    ElevateSandbox,
    #[strum(serialize = "sandbox-add-read-dir")]
    SandboxReadRoot,
    Experimental,
    Skills,
    Review,
    Rename,
    New,
    Resume,
    Fork,
    Init,
    Compact,
    Plan,
    Collab,
    Agent,
    // Undo,
    Diff,
    Mention,
    Status,
    DebugConfig,
    Statusline,
    Mcp,
    Apps,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    Clean,
    Personality,
    TestApproval,
    // Debugging commands.
    #[strum(serialize = "debug-m-drop")]
    MemoryDrop,
    #[strum(serialize = "debug-m-update")]
    MemoryUpdate,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Feedback => "向维护者发送日志",
            SlashCommand::New => "在当前会话中开启新聊天",
            SlashCommand::Init => "创建包含 Codex 指令的 AGENTS.md 文件",
            SlashCommand::Compact => "总结对话以避免触达上下文上限",
            SlashCommand::Review => "审查当前改动并找出问题",
            SlashCommand::Rename => "重命名当前会话",
            SlashCommand::Resume => "恢复已保存会话",
            SlashCommand::Fork => "基于当前会话创建分支会话",
            // SlashCommand::Undo => "ask Codex to undo a turn",
            SlashCommand::Quit | SlashCommand::Exit => "退出 Codex",
            SlashCommand::Diff => "显示 git diff（包含未跟踪文件）",
            SlashCommand::Mention => "引用文件",
            SlashCommand::Skills => "使用 skills 提升 Codex 在特定任务上的表现",
            SlashCommand::Status => "显示当前会话配置与 token 使用情况",
            SlashCommand::DebugConfig => "显示配置层级与 requirement 来源（调试用）",
            SlashCommand::Statusline => "配置状态栏显示项",
            SlashCommand::Ps => "列出后台终端",
            SlashCommand::Clean => "停止所有后台终端",
            SlashCommand::MemoryDrop => "请勿使用",
            SlashCommand::MemoryUpdate => "请勿使用",
            SlashCommand::Model => "选择模型与推理强度",
            SlashCommand::Personality => "选择 Codex 的沟通风格",
            SlashCommand::Plan => "切换到 Plan 模式",
            SlashCommand::Collab => "切换协作模式（实验性）",
            SlashCommand::Agent => "切换当前 agent 线程",
            SlashCommand::Approvals => "设置 Codex 的权限范围",
            SlashCommand::Permissions => "设置 Codex 的权限范围",
            SlashCommand::ElevateSandbox => "配置高权限 agent 沙箱",
            SlashCommand::SandboxReadRoot => "允许沙箱读取目录：/sandbox-add-read-dir <绝对路径>",
            SlashCommand::Experimental => "切换实验特性",
            SlashCommand::Mcp => "列出已配置的 MCP 工具",
            SlashCommand::Apps => "管理应用",
            SlashCommand::Logout => "退出 Codex 账号",
            SlashCommand::Rollout => "输出 rollout 文件路径",
            SlashCommand::TestApproval => "测试审批请求",
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Rename
                | SlashCommand::Plan
                | SlashCommand::SandboxReadRoot
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Resume
            | SlashCommand::Fork
            | SlashCommand::Init
            | SlashCommand::Compact
            // | SlashCommand::Undo
            | SlashCommand::Model
            | SlashCommand::Personality
            | SlashCommand::Approvals
            | SlashCommand::Permissions
            | SlashCommand::ElevateSandbox
            | SlashCommand::SandboxReadRoot
            | SlashCommand::Experimental
            | SlashCommand::Review
            | SlashCommand::Plan
            | SlashCommand::Logout
            | SlashCommand::MemoryDrop
            | SlashCommand::MemoryUpdate => false,
            SlashCommand::Diff
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Status
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Clean
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Feedback
            | SlashCommand::Quit
            | SlashCommand::Exit => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Collab => true,
            SlashCommand::Agent => true,
            SlashCommand::Statusline => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::SandboxReadRoot => cfg!(target_os = "windows"),
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}
