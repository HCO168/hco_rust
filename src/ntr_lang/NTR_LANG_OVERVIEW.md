# ntr_lang 代码说明（当前版本）

本文档描述 `src/ntr_lang` 目录下自然语言模块的当前实现。

## 1. 模块结构

- `mod.rs`
  - 暴露 `lang`、`core`、`en`、`zh_cn`、`chinese` 子模块。
  - 含基础单元测试，覆盖中英文句子渲染与词表加载。
- `core.rs`
  - 定义核心语义结构（如 `Clause`、`Tense`、`Aspect`、`Modifier`）。
  - 提供词表 JSON 加载辅助函数。
- `lang.rs`
  - 通用语言抽象与基础接口定义。
- `en/`
  - 英文渲染实现与英文词表（名词/动词）。
  - 包含规则动词过去式处理与不规则动词表。
- `zh_cn/`
  - 中文渲染实现与中文词表（名词/动词）。
  - `location_suffix.json`：地点固定搭配后缀表（如“天空-中”“公交车-上”）。
- `res/`
  - 词条 key 的资源文件（noun/verb key）。
- `chinese.rs`
  - 数字转中文模块（与句子渲染并存于 `ntr_lang` 目录下）。

## 2. 当前句子生成流程

1. 业务代码构造 `Clause`（主语、谓词、宾语、时态、体、否定、修饰语等）。
2. 目标语言包（如 `en::load()` / `zh_cn::load()`）通过 JSON 词表将语义 key 映射为词面值。
3. 对 `Clause` 执行语言特定线性化（word order + 语法标记），产出最终句子字符串。

词条查找新增继承机制：

- `LangPack` 包含 `parent_lang` 字段。
- 查词顺序为：当前语言 -> `parent_lang`（递归）-> 原始 key。
- 当前配置：
  - `en`：`parent_lang = None`
  - `zh_cn`：`parent_lang = en`（中文缺词时回退英文）

## 2.1 网站本地化方案（新增）

为兼容网站 UI 文案与动态句子，本模块采用“原子词汇 + 句子组合”方案：

1. 原子词汇（推荐用于按钮、标签、状态、连接词）
   - 放在 `noun_key.json` + `*/noun_value.json`。
   - 示例：`ui.username`、`ui.password`、`connector.or`、`state.invalid`。
2. 动态句子（推荐用于可组合语义）
   - 可用 `compose_sentence` 按 key 序列组句。
   - 示例：`[ui.username, connector.or, ui.password, connector.is, state.invalid]`。
   - 用 `Clause` 组合主语/谓语/修饰语。
   - 示例：`Clause::new("web.requested_file", "verb.exist").negative()`。

## 3. 中文（`zh_cn`）当前规则

`ChinesePack::render` 的当前主干规则：

- 时间修饰语：前置输出，格式为 `时间，`。
- 主语后若为进行体且非否定：输出 `正在`。
- 位置修饰语：输出为 `在 + 地点`。
  - 若 `location_suffix.json` 中存在该地点 key，则继续追加后缀（如 `中/上`）。
- 方式修饰语：输出为 `方式 + 地`。
- 否定：
  - 过去时用 `没`
  - 其他时态用 `不`
- 动词后接宾语（若有）。
- 若满足完成/过去条件（且非否定）追加 `了`。
- 句末追加 `。`。

`verb.transfer`（“给”字句）单独走 `render_transfer` 分支，处理受词/宾语位置与“给了”形态。

## 4. 本次语序修复

针对进行体 + 地点修饰语，将中文语序从：

- `Alice在天空正在飞。`

修复为：

- `Alice正在在天空中飞。`

对应实现方式：把 `正在` 的输出位置提前到修饰语遍历之前，使其出现在主语之后。

## 5. 已知边界（当前未扩展）

- 目前中文进行体在“在 + 地点”场景采用的是机械规则拼接，未做更细粒度风格化变体（如“正在天空中飞”）。
- 复合从句、连词逻辑、量词与更复杂语义角色尚未系统化。
- 词表驱动为主，尚无上下文语义消歧层。

## 6. 网站词条命名约定（新增）

- `ui.*`：通用界面词汇（字段名、按钮名、导航项）。
- `web.*`：Web 领域实体（文件、页面、服务器、网络等）。
- `connector.*`：连接词与语法胶水（如 `or`、`is`）。
- `state.*`：状态词（如 `invalid`、`not_found`、`expired`）。
- `action.*`：动作词（如 `save`、`load`、`try_again`）。
- `verb.*`：可参与句子组合的动词语义 key。

已补充的高频网站词条包括：

- 词汇：`ui.user`、`ui.username`、`ui.password`、`ui.login`、`ui.logout`、`ui.register`、`ui.settings` 等。
- 状态/连接词：`state.invalid`、`state.not_found`、`state.unavailable`、`connector.or`、`connector.is` 等。
- 组合句支持：`compose_sentence` 可生成“用户名或密码无效。”；`Clause` 可生成“你请求的文件不存在。”。
- 继承回退示例：`ui.remember_me` 当前仅在英文词表定义，中文读取时会回退到英文 `"Remember me"`。
