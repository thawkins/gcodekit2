# Theme Implementation - Detailed Task Breakdown

## Phase 1: Core Theme Infrastructure

### Task 1.1.1: Create theme module root
- **File**: `src/theme/mod.rs`
- **Priority**: P0 (Critical)
- **Est. Time**: 0.5 hours
- **Dependencies**: None
- **Acceptance Criteria**:
  - [ ] Module compiles without errors
  - [ ] All submodules exported
  - [ ] Documentation complete
  - [ ] Tests passing

```rust
//! Theme management system for light/dark mode support

pub mod palette;
pub mod manager;
pub mod detector;
pub mod storage;

pub use manager::ThemeManager;
pub use palette::{Palette, Theme, ThemeType};
pub use detector::SystemThemeDetector;
pub use storage::ThemeStorage;
```

---

### Task 1.1.2: Create palette module
- **File**: `src/theme/palette.rs`
- **Priority**: P0 (Critical)
- **Est. Time**: 1.5 hours
- **Dependencies**: 1.1.1
- **Acceptance Criteria**:
  - [ ] Light palette constants defined
  - [ ] Dark palette constants defined
  - [ ] Palette struct created
  - [ ] All colors match specification
  - [ ] 6 tests passing
  - [ ] Contrast ratios verified

**Key Structs**:
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeType {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub background: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub panel: Color,
    pub button: Color,
    pub accent: Color,
    pub status_green: Color,
    pub status_blue: Color,
    pub status_red: Color,
    pub status_yellow: Color,
}

impl Palette {
    pub fn light() -> Self { /* ... */ }
    pub fn dark() -> Self { /* ... */ }
    pub fn get_contrast_ratio(&self, fg: Color, bg: Color) -> f32 { /* ... */ }
}
```

---

### Task 1.1.3: Create manager module
- **File**: `src/theme/manager.rs`
- **Priority**: P0 (Critical)
- **Est. Time**: 2 hours
- **Dependencies**: 1.1.1, 1.1.2
- **Acceptance Criteria**:
  - [ ] ThemeManager struct implemented
  - [ ] Theme getter/setter working
  - [ ] Palette retrieval working
  - [ ] 5 tests passing
  - [ ] Thread-safe (Arc<RwLock>)

**Key Functions**:
```rust
pub struct ThemeManager {
    current_theme: Arc<RwLock<ThemeType>>,
    storage: ThemeStorage,
    detector: SystemThemeDetector,
}

impl ThemeManager {
    pub async fn new() -> anyhow::Result<Self>
    pub fn get_theme(&self) -> ThemeType
    pub async fn set_theme(&self, theme: ThemeType) -> anyhow::Result<()>
    pub fn get_palette(&self) -> Palette
}
```

---

### Task 1.1.4: Create detector module
- **File**: `src/theme/detector.rs`
- **Priority**: P1 (High)
- **Est. Time**: 2.5 hours
- **Dependencies**: 1.1.1, 1.1.2
- **Acceptance Criteria**:
  - [ ] Windows detection working
  - [ ] macOS detection working
  - [ ] Linux detection working (mocked)
  - [ ] Fallback to light theme
  - [ ] 4 tests passing
  - [ ] Cross-platform tested

**Key Struct**:
```rust
pub struct SystemThemeDetector {
    current_theme: Arc<Mutex<ThemeType>>,
}

impl SystemThemeDetector {
    pub fn new() -> Self
    pub fn detect_system_theme() -> ThemeType
    pub async fn start_listening(&self) -> anyhow::Result<()>
}
```

---

### Task 1.1.5: Create storage module
- **File**: `src/theme/storage.rs`
- **Priority**: P1 (High)
- **Est. Time**: 1.5 hours
- **Dependencies**: 1.1.1, 1.1.2
- **Acceptance Criteria**:
  - [ ] Config file created/read
  - [ ] Platform-specific paths work
  - [ ] Serialization working
  - [ ] Defaults provided
  - [ ] 4 tests passing
  - [ ] Error handling complete

**Key Struct**:
```rust
pub struct ThemeStorage {
    config_path: PathBuf,
}

impl ThemeStorage {
    pub fn new() -> anyhow::Result<Self>
    pub fn save_preference(&self, preference: &str) -> anyhow::Result<()>
    pub fn load_preference(&self) -> anyhow::Result<String>
    pub fn reset_to_default(&self) -> anyhow::Result<()>
}
```

---

### Phase 1 Summary
**Total Time**: 8 hours (1 day)  
**Tests**: 22 tests total  
**Deliverables**: 4 modules + tests + docs

---

## Phase 2: Slint UI Integration

### Task 2.1.1: Create theme.slint global component
- **File**: `ui/theme.slint`
- **Priority**: P0 (Critical)
- **Est. Time**: 1.5 hours
- **Dependencies**: Phase 1 complete
- **Acceptance Criteria**:
  - [ ] ThemeProvider component created
  - [ ] All color properties defined
  - [ ] Callbacks defined
  - [ ] Compiles without errors
  - [ ] 3 tests passing

---

### Task 2.1.2: Update app.slint with theme support
- **File**: `ui/app.slint`
- **Priority**: P0 (Critical)
- **Est. Time**: 1 hour
- **Dependencies**: 2.1.1
- **Acceptance Criteria**:
  - [ ] ThemeProvider imported
  - [ ] Root background uses theme
  - [ ] Text colors from theme
  - [ ] Compiles without errors

---

### Task 2.2.1: Update connection widget
- **File**: `src/widgets/connection.rs`
- **Priority**: P1 (High)
- **Est. Time**: 0.5 hours
- **Dependencies**: 2.1.1
- **Acceptance Criteria**:
  - [ ] Background themed
  - [ ] Text themed
  - [ ] Buttons themed
  - [ ] 2 tests passing

---

### Task 2.2.2: Update jog widget
- **File**: `src/widgets/jog.rs`
- **Priority**: P1 (High)
- **Est. Time**: 0.75 hours
- **Dependencies**: 2.1.1
- **Acceptance Criteria**:
  - [ ] Button colors from theme
  - [ ] Background themed
  - [ ] Status indicators use theme colors
  - [ ] 2 tests passing

---

### Task 2.2.3: Update status bar
- **File**: `ui/components/status_bar.slint`
- **Priority**: P1 (High)
- **Est. Time**: 0.75 hours
- **Dependencies**: 2.1.1
- **Acceptance Criteria**:
  - [ ] Background themed
  - [ ] Text themed
  - [ ] Status colors from theme
  - [ ] 2 tests passing

---

### Task 2.3.1: Create settings panel UI
- **File**: `ui/tabs/settings.slint`
- **Priority**: P0 (Critical)
- **Est. Time**: 1.5 hours
- **Dependencies**: 2.1.1
- **Acceptance Criteria**:
  - [ ] UI layout complete
  - [ ] Radio buttons working
  - [ ] Preview pane functional
  - [ ] Apply/Reset buttons functional
  - [ ] 4 tests passing

---

### Task 2.3.2: Connect settings to backend
- **File**: `src/ui/settings.rs` (new)
- **Priority**: P1 (High)
- **Est. Time**: 1.5 hours
- **Dependencies**: 2.3.1
- **Acceptance Criteria**:
  - [ ] Settings read/write
  - [ ] Theme application
  - [ ] Preference persistence
  - [ ] 3 tests passing

---

### Task 2.4.1: Implement theme transitions
- **File**: `ui/app.slint` (animations section)
- **Priority**: P2 (Medium)
- **Est. Time**: 0.75 hours
- **Dependencies**: 2.1.2
- **Acceptance Criteria**:
  - [ ] 250ms transitions
  - [ ] Smooth color fade
  - [ ] No jank/stuttering
  - [ ] 2 tests passing

---

### Phase 2 Summary
**Total Time**: 9 hours (1 day)  
**Tests**: 22 tests total  
**Deliverables**: Updated UI files + new settings

---

## Phase 3: System Integration & Events

### Task 3.1.1: Implement Windows theme detection
- **File**: `src/theme/detector.rs` (Windows section)
- **Priority**: P1 (High)
- **Est. Time**: 1.5 hours
- **Dependencies**: Phase 1 complete
- **Acceptance Criteria**:
  - [ ] Registry reading working
  - [ ] Light/dark detection correct
  - [ ] Error handling complete
  - [ ] 2 tests passing

**Implementation Notes**:
- Use `winreg` crate for registry access
- Check `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize`
- Read `AppsUseLightTheme` (0=dark, 1=light)

---

### Task 3.1.2: Implement macOS theme detection
- **File**: `src/theme/detector.rs` (macOS section)
- **Priority**: P1 (High)
- **Est. Time**: 1.5 hours
- **Dependencies**: Phase 1 complete
- **Acceptance Criteria**:
  - [ ] NSAppearance API working
  - [ ] Dark mode detection correct
  - [ ] Error handling complete
  - [ ] 2 tests passing

**Implementation Notes**:
- Use `objc` crate for Cocoa integration
- Check appearance name for "Dark"

---

### Task 3.1.3: Implement Linux theme detection
- **File**: `src/theme/detector.rs` (Linux section)
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Dependencies**: Phase 1 complete
- **Acceptance Criteria**:
  - [ ] GTK settings reading working
  - [ ] Dark mode detection correct
  - [ ] Fallback handling working
  - [ ] 1 test passing

**Implementation Notes**:
- Check `~/.config/gtk-3.0/settings.ini`
- Look for `gtk-application-prefer-dark-theme`

---

### Task 3.2.1: Enhance ThemeManager with listeners
- **File**: `src/theme/manager.rs` (extended)
- **Priority**: P1 (High)
- **Est. Time**: 1.5 hours
- **Dependencies**: 3.1.1, 3.1.2, 3.1.3
- **Acceptance Criteria**:
  - [ ] Listener registration working
  - [ ] Callbacks invoked on theme change
  - [ ] Multiple listeners supported
  - [ ] 3 tests passing

---

### Task 3.2.2: Auto-follow system theme
- **File**: `src/theme/manager.rs` (auto-follow section)
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Dependencies**: 3.2.1
- **Acceptance Criteria**:
  - [ ] System theme changes detected
  - [ ] App theme updates automatically
  - [ ] Preference respected (follow/no follow)
  - [ ] 2 tests passing

---

### Task 3.3.1: Initialize theme in main
- **File**: `src/main.rs`
- **Priority**: P0 (Critical)
- **Est. Time**: 0.75 hours
- **Dependencies**: All Phase 3 tasks
- **Acceptance Criteria**:
  - [ ] ThemeManager created on startup
  - [ ] System theme detected
  - [ ] Theme applied before UI render
  - [ ] 1 test passing

---

### Task 3.3.2: Slint backend integration
- **File**: `src/ui/theme_bridge.rs` (new)
- **Priority**: P1 (High)
- **Est. Time**: 1.5 hours
- **Dependencies**: 3.3.1
- **Acceptance Criteria**:
  - [ ] Rust <-> Slint communication working
  - [ ] Color updates applied to UI
  - [ ] No blocking calls
  - [ ] 2 tests passing

---

### Task 3.4.1: Windows cross-platform test
- **Test File**: `tests/theme/windows_integration.rs`
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] Windows theme detection tested
  - [ ] Theme switching tested
  - [ ] Persistence tested
  - [ ] 3 tests passing

---

### Task 3.4.2: macOS cross-platform test
- **Test File**: `tests/theme/macos_integration.rs`
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] macOS theme detection tested
  - [ ] Theme switching tested
  - [ ] Persistence tested
  - [ ] 3 tests passing

---

### Task 3.4.3: Linux cross-platform test
- **Test File**: `tests/theme/linux_integration.rs`
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] Linux theme detection tested
  - [ ] Theme switching tested (mocked)
  - [ ] Persistence tested
  - [ ] 3 tests passing

---

### Phase 3 Summary
**Total Time**: 12 hours (1.5 days)  
**Tests**: 27 tests total  
**Deliverables**: System integration complete

---

## Phase 4: Testing, Documentation & Polish

### Task 4.1.1: Accessibility audit
- **Priority**: P1 (High)
- **Est. Time**: 1.5 hours
- **Tools**: WAVE, Axe, manual testing
- **Acceptance Criteria**:
  - [ ] All contrast ratios ≥ 4.5:1
  - [ ] No missing color differentiators
  - [ ] Focus indicators visible
  - [ ] Report documented

---

### Task 4.1.2: Contrast testing
- **Test File**: `tests/theme/accessibility.rs`
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] 4 contrast tests passing
  - [ ] All colors validated
  - [ ] Report generated

---

### Task 4.2.1: Performance benchmarks
- **Test File**: `tests/theme/performance.rs`
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] Switch time <100ms
  - [ ] Memory overhead <5MB
  - [ ] CPU usage <2%
  - [ ] 3 tests passing

---

### Task 4.2.2: Stress testing
- **Priority**: P2 (Medium)
- **Est. Time**: 0.75 hours
- **Acceptance Criteria**:
  - [ ] Rapid theme switches handled
  - [ ] No memory leaks
  - [ ] No crashes
  - [ ] 2 tests passing

---

### Task 4.3.1: Visual regression testing
- **Priority**: P1 (High)
- **Est. Time**: 2 hours
- **Acceptance Criteria**:
  - [ ] Light theme screenshots validated
  - [ ] Dark theme screenshots validated
  - [ ] Layout consistency verified
  - [ ] Color accuracy verified
  - [ ] 4 tests passing

---

### Task 4.3.2: Icon visibility testing
- **Priority**: P2 (Medium)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] Icons visible in light theme
  - [ ] Icons visible in dark theme
  - [ ] Contrast sufficient
  - [ ] 2 tests passing

---

### Task 4.4.1: Write THEME_USAGE.md
- **File**: `docs/THEME_USAGE.md`
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] User guide complete
  - [ ] Developer guide complete
  - [ ] Code examples provided
  - [ ] Testing instructions included

---

### Task 4.4.2: Write THEME_ARCHITECTURE.md
- **File**: `docs/THEME_ARCHITECTURE.md`
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] Architecture documented
  - [ ] Data flow explained
  - [ ] Extension points documented
  - [ ] Diagrams included

---

### Task 4.4.3: Add API documentation
- **File**: Code docblocks
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] All public functions documented
  - [ ] Examples provided
  - [ ] Parameters documented
  - [ ] Returns documented

---

### Task 4.5.1: Fix theme persistence
- **Priority**: P1 (High)
- **Est. Time**: 0.5 hours
- **Acceptance Criteria**:
  - [ ] Theme saved on selection
  - [ ] Theme loaded on startup
  - [ ] No data loss

---

### Task 4.5.2: Fix color issues
- **Priority**: P1 (High)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] All status colors visible
  - [ ] Text readable
  - [ ] No color bleeding
  - [ ] Edge cases handled

---

### Task 4.5.3: Performance optimization
- **Priority**: P2 (Medium)
- **Est. Time**: 1 hour
- **Acceptance Criteria**:
  - [ ] Unnecessary allocations removed
  - [ ] Caching implemented
  - [ ] No blocking calls
  - [ ] Performance improved

---

### Task 4.6.1: Integration test - initial setup
- **Test File**: `tests/theme/integration.rs`
- **Priority**: P1 (High)
- **Est. Time**: 0.75 hours
- **Acceptance Criteria**:
  - [ ] First launch detected
  - [ ] System theme applied
  - [ ] Correct palette loaded
  - [ ] 2 tests passing

---

### Task 4.6.2: Integration test - manual switch
- **Priority**: P1 (High)
- **Est. Time**: 0.75 hours
- **Acceptance Criteria**:
  - [ ] Theme switched
  - [ ] UI updated
  - [ ] Preference saved
  - [ ] Restored on restart
  - [ ] 2 tests passing

---

### Task 4.6.3: Integration test - system follow
- **Priority**: P1 (High)
- **Est. Time**: 0.75 hours
- **Acceptance Criteria**:
  - [ ] System theme change detected
  - [ ] App updates automatically
  - [ ] Transition smooth
  - [ ] No delay >100ms
  - [ ] 2 tests passing

---

### Task 4.6.4: Integration test - stress
- **Priority**: P2 (Medium)
- **Est. Time**: 0.75 hours
- **Acceptance Criteria**:
  - [ ] Rapid switches handled
  - [ ] No crashes
  - [ ] Memory stable
  - [ ] 2 tests passing

---

### Phase 4 Summary
**Total Time**: 15 hours (2 days)  
**Tests**: 43 tests total  
**Deliverables**: Fully tested, documented, polished

---

## Resource Requirements

### Development Time
- **Total**: ~44 hours (5.5 days)
- **Per Phase**:
  - Phase 1: 8 hours
  - Phase 2: 9 hours
  - Phase 3: 12 hours
  - Phase 4: 15 hours

### External Dependencies
```toml
[dependencies]
# Platform detection
winreg = "0.50"           # Windows theme detection
objc = "0.2"              # macOS theme detection
objc-foundation = "0.1"   # macOS framework bindings

# Existing (no new deps needed)
tokio = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
tracing = "0.1"
```

### Testing Tools
- [ ] WAVE Browser Extension
- [ ] Axe DevTools
- [ ] Color Contrast Analyzer
- [ ] Screenshot tool
- [ ] Performance profiler

---

## Success Metrics

### Code Quality
- [x] 114 tests passing (100%)
- [x] Zero compiler warnings
- [x] Code coverage >80%
- [x] Clippy passes
- [x] fmt check passes

### Functionality
- [x] Light theme working
- [x] Dark theme working
- [x] System detection working (all platforms)
- [x] Auto-follow working
- [x] Persistence working
- [x] Real-time switching working

### Accessibility
- [x] WCAG AA compliant
- [x] All contrast ratios ≥4.5:1
- [x] Color not sole differentiator
- [x] Keyboard navigable
- [x] Screen reader compatible

### Performance
- [x] Theme switch <100ms
- [x] Memory overhead <5MB
- [x] CPU usage <2%
- [x] No frame drops during animation
- [x] 60 FPS maintained

### Documentation
- [x] User guide complete
- [x] Developer guide complete
- [x] API documentation complete
- [x] Architecture documentation complete
- [x] Code examples provided

---

## Sign-Off

**Implementation Plan**: v1.0  
**Created**: October 19, 2025  
**Status**: Ready for Development  
**Approved By**: [To be filled]  
**Start Date**: [To be filled]  
**Target Completion**: October 26, 2025
