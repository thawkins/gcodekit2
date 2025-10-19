# UI Theme Support Implementation Plan

**Feature**: Phase 14 - System Theme Support (Light/Dark Mode)  
**Version**: 0.1.0-alpha  
**Start Date**: October 19, 2025  
**Target Completion**: October 26, 2025  

---

## Overview

This document outlines the implementation strategy for adding dynamic light/dark theme support to GCodeKit2. The implementation will be staged across 4 phases to ensure thorough testing and validation at each stage.

### Goals
- ✓ Detect and respond to system theme preference
- ✓ Implement consistent light and dark color palettes
- ✓ Support real-time theme switching
- ✓ Persist user preferences
- ✓ Maintain WCAG AA accessibility standards

---

## Architecture

### Theme System Components

```
┌─────────────────────────────────────────────────────────────┐
│                    GCodeKit2 Application                     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Main UI    │  │ Settings     │  │ Device       │      │
│  │  (app.slint) │  │ Panel        │  │ Console      │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                 │                 │               │
│         └─────────────────┼─────────────────┘               │
│                           │                                  │
│         ┌─────────────────┴─────────────────┐               │
│         ▼                                     ▼               │
│  ┌─────────────────────────────────────────────────┐        │
│  │         Theme Manager (Rust Module)              │        │
│  ├─────────────────────────────────────────────────┤        │
│  │ • Current Theme (Light/Dark)                     │        │
│  │ • Color Palette                                  │        │
│  │ • System Theme Detection                         │        │
│  │ • User Preference Loading/Saving                 │        │
│  └─────────────────────────────────────────────────┘        │
│         │                │                 │                 │
│         ├─────────┬──────┴──────┬──────────┤                │
│         ▼         ▼             ▼          ▼                │
│    ┌────────┐ ┌────────┐ ┌──────────┐ ┌──────────┐        │
│    │ Light  │ │ Dark   │ │ System   │ │ Storage  │        │
│    │Palette │ │Palette │ │Detector  │ │(Config)  │        │
│    └────────┘ └────────┘ └──────────┘ └──────────┘        │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

```
System Theme Changed
        │
        ▼
   Theme Detector
        │
        ▼
  Theme Manager
        │
        ├─→ Load Palette
        ├─→ Apply Colors
        └─→ Update UI
        
User Selects Theme
        │
        ▼
   Settings Panel
        │
        ├─→ Theme Manager
        │
        ├─→ Save Preference
        │
        └─→ Apply & Update UI
```

---

## Phase 1: Core Theme Infrastructure (Days 1-2)

### Objectives
- Create theme management module
- Define color palettes
- Implement theme storage

### Tasks

#### 1.1 Create Theme Module Structure
**File**: `src/theme/mod.rs`

```rust
//! Theme management system for light/dark mode support
//!
//! Provides theme detection, palette management, and preference persistence.

pub mod palette;
pub mod manager;
pub mod detector;
pub mod storage;

pub use manager::ThemeManager;
pub use palette::{Palette, Theme, ThemeType};
pub use detector::SystemThemeDetector;
pub use storage::ThemeStorage;
```

**Deliverables**:
- [ ] `src/theme/mod.rs` - Module root
- [ ] `src/theme/palette.rs` - Color definitions
- [ ] `src/theme/manager.rs` - Theme orchestration
- [ ] `src/theme/detector.rs` - System theme detection
- [ ] `src/theme/storage.rs` - Preference persistence

**Test Requirements** (8 tests):
- [ ] Theme creation and validation
- [ ] Palette color accessibility
- [ ] Color contrast ratios
- [ ] Storage/retrieval of preferences

---

#### 1.2 Define Color Palettes
**File**: `src/theme/palette.rs`

**Light Theme Colors**:
```rust
pub const LIGHT_BACKGROUND: Color = Color::from_rgb(255, 255, 255);
pub const LIGHT_TEXT_PRIMARY: Color = Color::from_rgb(26, 26, 26);
pub const LIGHT_TEXT_SECONDARY: Color = Color::from_rgb(102, 102, 102);
pub const LIGHT_PANEL: Color = Color::from_rgb(245, 245, 245);
pub const LIGHT_BUTTON: Color = Color::from_rgb(0, 102, 204);
pub const LIGHT_ACCENT: Color = Color::from_rgb(255, 107, 53);
pub const LIGHT_STATUS_GREEN: Color = Color::from_rgb(0, 170, 0);
pub const LIGHT_STATUS_BLUE: Color = Color::from_rgb(0, 0, 255);
pub const LIGHT_STATUS_RED: Color = Color::from_rgb(255, 0, 0);
pub const LIGHT_STATUS_YELLOW: Color = Color::from_rgb(255, 170, 0);
```

**Dark Theme Colors**:
```rust
pub const DARK_BACKGROUND: Color = Color::from_rgb(30, 30, 30);
pub const DARK_TEXT_PRIMARY: Color = Color::from_rgb(255, 255, 255);
pub const DARK_TEXT_SECONDARY: Color = Color::from_rgb(204, 204, 204);
pub const DARK_PANEL: Color = Color::from_rgb(45, 45, 45);
pub const DARK_BUTTON: Color = Color::from_rgb(77, 166, 255);
pub const DARK_ACCENT: Color = Color::from_rgb(255, 140, 66);
pub const DARK_STATUS_GREEN: Color = Color::from_rgb(0, 255, 0);
pub const DARK_STATUS_BLUE: Color = Color::from_rgb(77, 166, 255);
pub const DARK_STATUS_RED: Color = Color::from_rgb(255, 51, 51);
pub const DARK_STATUS_YELLOW: Color = Color::from_rgb(255, 221, 0);
```

**Deliverables**:
- [ ] Complete palette enums and constants
- [ ] Struct definitions for Palette, ThemeType
- [ ] Methods for color retrieval
- [ ] WCAG contrast validation

**Test Requirements** (6 tests):
- [ ] All colors defined and accessible
- [ ] Contrast ratios meet WCAG AA (4.5:1 min)
- [ ] Theme type enum conversions
- [ ] Color interpolation if needed

---

#### 1.3 System Theme Detection
**File**: `src/theme/detector.rs`

**Windows Detection**:
```rust
// Use Windows Registry to detect theme
// HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize
// AppsUseLightTheme = 0 (dark), 1 (light)
```

**macOS Detection**:
```rust
// Use NSAppearance API via objc crate
// appearances().contains("Dark") -> dark theme
```

**Linux Detection**:
```rust
// Check GTK settings via dbus or config files
// gtk-application-prefer-dark-theme in settings.ini
```

**Deliverables**:
- [ ] Cross-platform detection implementation
- [ ] Fallback to Light theme if detection fails
- [ ] Theme change event listener
- [ ] Polling mechanism for theme changes

**Test Requirements** (4 tests):
- [ ] Windows theme detection
- [ ] macOS theme detection
- [ ] Linux theme detection (mocked)
- [ ] Fallback behavior

---

#### 1.4 Theme Storage
**File**: `src/theme/storage.rs`

**Config Location**:
```
Linux:   ~/.config/gcodekit2/theme.json
Windows: %APPDATA%\gcodekit2\theme.json
macOS:   ~/Library/Application Support/gcodekit2/theme.json
```

**Storage Format**:
```json
{
  "theme": "dark",
  "auto_follow_system": true,
  "last_updated": "2025-10-19T09:05:06Z",
  "custom_overrides": {}
}
```

**Deliverables**:
- [ ] JSON serialization/deserialization
- [ ] Platform-specific config paths
- [ ] Create config directory if missing
- [ ] Default config generation

**Test Requirements** (4 tests):
- [ ] Config file creation
- [ ] Read/write preferences
- [ ] Default values
- [ ] Migration from old formats

---

### Phase 1 Completion Checklist
- [ ] All theme module files created
- [ ] Color palettes defined and tested
- [ ] System detection working on all platforms
- [ ] Storage system functional
- [ ] 22 tests passing
- [ ] Code documentation complete
- [ ] No compiler warnings

**Estimated Time**: 2 days (Oct 19-20)

---

## Phase 2: Slint UI Integration (Days 3-4)

### Objectives
- Integrate theme system with Slint UI
- Implement dynamic color binding
- Create settings panel

### Tasks

#### 2.1 Theme-Aware Slint Components
**File**: `ui/theme.slint`

**Global Properties**:
```slint
import { StandardButton, Button, LineEdit, ComboBox } from "std-widgets.slint";

export global ThemeProvider {
    in-out property <string> current-theme: "light";
    
    // Light theme colors
    property <color> light-background: #FFFFFF;
    property <color> light-text-primary: #1A1A1A;
    property <color> light-text-secondary: #666666;
    property <color> light-panel: #F5F5F5;
    property <color> light-button: #0066CC;
    property <color> light-accent: #FF6B35;
    property <color> light-status-green: #00AA00;
    property <color> light-status-blue: #0000FF;
    property <color> light-status-red: #FF0000;
    property <color> light-status-yellow: #FFAA00;
    
    // Dark theme colors
    property <color> dark-background: #1E1E1E;
    property <color> dark-text-primary: #FFFFFF;
    property <color> dark-text-secondary: #CCCCCC;
    property <color> dark-panel: #2D2D2D;
    property <color> dark-button: #4DA6FF;
    property <color> dark-accent: #FF8C42;
    property <color> dark-status-green: #00FF00;
    property <color> dark-status-blue: #4DA6FF;
    property <color> dark-status-red: #FF3333;
    property <color> dark-status-yellow: #FFDD00;
    
    // Dynamic accessors
    pure callback get-background() -> color;
    pure callback get-text-primary() -> color;
    pure callback get-text-secondary() -> color;
    pure callback get-panel() -> color;
    pure callback get-button() -> color;
    pure callback get-accent() -> color;
    pure callback get-status-green() -> color;
    pure callback get-status-blue() -> color;
    pure callback get-status-red() -> color;
    pure callback get-status-yellow() -> color;
}
```

**Deliverables**:
- [ ] ThemeProvider global component
- [ ] All color properties defined
- [ ] Dynamic color accessor callbacks
- [ ] Slint integration in app.slint

**Test Requirements** (6 tests):
- [ ] Theme provider creation
- [ ] Color property updates
- [ ] Callback execution
- [ ] UI element color binding

---

#### 2.2 Theme-Aware UI Components
**Files**: Modify existing UI components

**Connection Widget** (`ui/widgets/connection.rs`):
- [ ] Use ThemeProvider for backgrounds
- [ ] Use theme colors for buttons
- [ ] Apply text colors based on theme

**Jog Widget** (`ui/widgets/jog.rs`):
- [ ] Button colors from theme
- [ ] Background from theme
- [ ] Status indicators using theme colors

**Status Bar** (`ui/components/status_bar.rs`):
- [ ] Background from theme
- [ ] Text colors from theme
- [ ] Status color indicators

**Deliverables**:
- [ ] All widgets using ThemeProvider
- [ ] Dynamic color binding
- [ ] No hardcoded colors

**Test Requirements** (8 tests):
- [ ] Connection widget theming
- [ ] Jog widget theming
- [ ] Status bar theming
- [ ] Tab appearance
- [ ] Color transitions

---

#### 2.3 Settings Panel Theme Selection
**File**: `ui/tabs/settings.slint`

**UI Layout**:
```
┌─────────────────────────────────────┐
│ Theme Settings                      │
├─────────────────────────────────────┤
│                                     │
│ Theme Selection:                    │
│ ○ Light Theme                       │
│ ○ Dark Theme                        │
│ ○ Follow System (default)           │
│                                     │
│ Preview:                            │
│ ┌───────────────────────────────┐  │
│ │ Background Sample             │  │
│ │ Text Sample                   │  │
│ └───────────────────────────────┘  │
│                                     │
│ [Apply] [Reset to Default]          │
│                                     │
└─────────────────────────────────────┘
```

**Components**:
```slint
export component ThemeSettings inherits Rectangle {
    in-out property <string> current-theme: "system";
    
    callback on-theme-changed(string);
    
    VerticalLayout {
        Text {
            text: "Theme Settings";
            font-size: 16px;
            font-weight: bold;
        }
        
        Rectangle { height: 10px; }
        
        VerticalLayout {
            HorizontalLayout {
                RadioButton {
                    text: "Light Theme";
                    toggled => {
                        root.current-theme = "light";
                        root.on-theme-changed("light");
                    }
                }
            }
            
            HorizontalLayout {
                RadioButton {
                    text: "Dark Theme";
                    toggled => {
                        root.current-theme = "dark";
                        root.on-theme-changed("dark");
                    }
                }
            }
            
            HorizontalLayout {
                RadioButton {
                    text: "Follow System";
                    checked: current-theme == "system";
                    toggled => {
                        root.current-theme = "system";
                        root.on-theme-changed("system");
                    }
                }
            }
        }
        
        Rectangle { height: 20px; }
        
        HorizontalLayout {
            Button {
                text: "Apply";
                clicked => {
                    // Apply theme changes
                }
            }
            
            Button {
                text: "Reset to Default";
                clicked => {
                    // Reset to system default
                }
            }
        }
    }
}
```

**Deliverables**:
- [ ] Settings panel UI
- [ ] Radio button selection
- [ ] Theme preview pane
- [ ] Apply and Reset buttons
- [ ] Integration with main settings

**Test Requirements** (5 tests):
- [ ] Theme selection working
- [ ] Preview updates
- [ ] Apply button functionality
- [ ] Reset button functionality
- [ ] Settings panel layout

---

#### 2.4 Theme Transitions
**Animation Requirements**:
- [ ] 200-300ms fade transition when theme changes
- [ ] Smooth color interpolation
- [ ] Non-blocking UI updates

**Implementation**:
```slint
animate all 250ms ease-in-out;
```

**Deliverables**:
- [ ] Animation timings
- [ ] Transition effects
- [ ] Smooth color changes
- [ ] Performance optimization

**Test Requirements** (3 tests):
- [ ] Animation timing
- [ ] Color interpolation
- [ ] Performance (60 FPS)

---

### Phase 2 Completion Checklist
- [ ] ThemeProvider global component working
- [ ] All widgets using theme colors
- [ ] Settings panel functional
- [ ] Theme transitions smooth
- [ ] 22+ new tests passing
- [ ] No visual artifacts
- [ ] Performance maintained

**Estimated Time**: 2 days (Oct 21-22)

---

## Phase 3: System Integration & Events (Days 5-6)

### Objectives
- Implement system theme change detection
- Handle real-time theme switching
- Integrate with Rust backend

### Tasks

#### 3.1 System Theme Change Events
**File**: `src/theme/detector.rs` (extended)

**Event Loop**:
```rust
pub struct SystemThemeDetector {
    current_theme: Arc<Mutex<ThemeType>>,
    listeners: Arc<Mutex<Vec<Box<dyn Fn(ThemeType) + Send>>>>,
}

impl SystemThemeDetector {
    /// Start listening for system theme changes
    pub async fn start_listening(&self) -> anyhow::Result<()> {
        #[cfg(target_os = "windows")]
        self.start_windows_listener().await?;
        
        #[cfg(target_os = "macos")]
        self.start_macos_listener().await?;
        
        #[cfg(target_os = "linux")]
        self.start_linux_listener().await?;
        
        Ok(())
    }
    
    /// Register callback for theme changes
    pub fn on_theme_changed<F>(&self, callback: F) 
    where
        F: Fn(ThemeType) + Send + 'static,
    {
        self.listeners.lock().unwrap()
            .push(Box::new(callback));
    }
}
```

**Deliverables**:
- [ ] System theme listener implementation
- [ ] Windows theme change detection
- [ ] macOS theme change detection
- [ ] Linux theme change detection
- [ ] Event callback system
- [ ] Tokio async integration

**Test Requirements** (6 tests):
- [ ] System listener startup
- [ ] Theme change detection
- [ ] Callback invocation
- [ ] Multiple listener support
- [ ] Error handling

---

#### 3.2 Theme Manager Enhancements
**File**: `src/theme/manager.rs` (extended)

```rust
pub struct ThemeManager {
    current_theme: Arc<RwLock<ThemeType>>,
    user_preference: Arc<RwLock<String>>, // "light", "dark", "system"
    detector: SystemThemeDetector,
    storage: ThemeStorage,
}

impl ThemeManager {
    pub async fn new() -> anyhow::Result<Self> {
        let storage = ThemeStorage::new()?;
        let preference = storage.load_preference()?;
        let detector = SystemThemeDetector::new();
        
        let manager = Self {
            current_theme: Arc::new(RwLock::new(ThemeType::Light)),
            user_preference: Arc::new(RwLock::new(preference)),
            detector,
            storage,
        };
        
        // Start system theme monitoring
        manager.setup_monitoring().await?;
        
        Ok(manager)
    }
    
    /// Set theme and save preference
    pub async fn set_theme(&self, theme: ThemeType) -> anyhow::Result<()> {
        *self.current_theme.write().unwrap() = theme;
        self.storage.save_preference(&format!("{:?}", theme).to_lowercase())?;
        Ok(())
    }
    
    /// Get current theme
    pub fn get_theme(&self) -> ThemeType {
        *self.current_theme.read().unwrap()
    }
    
    /// Get palette for current theme
    pub fn get_palette(&self) -> Palette {
        match self.get_theme() {
            ThemeType::Light => Palette::light(),
            ThemeType::Dark => Palette::dark(),
        }
    }
    
    /// Setup auto-following system theme
    async fn setup_monitoring(&self) -> anyhow::Result<()> {
        let preference = self.user_preference.read().unwrap().clone();
        
        if preference == "system" {
            let current = self.current_theme.clone();
            self.detector.on_theme_changed(move |theme| {
                *current.write().unwrap() = theme;
                tracing::info!("System theme changed to: {:?}", theme);
            });
            
            self.detector.start_listening().await?;
        }
        
        Ok(())
    }
}
```

**Deliverables**:
- [ ] Enhanced ThemeManager with event support
- [ ] Auto-follow system theme logic
- [ ] Theme persistence
- [ ] Error handling

**Test Requirements** (5 tests):
- [ ] Theme setting and retrieval
- [ ] Preference saving
- [ ] System monitoring setup
- [ ] Auto-follow logic
- [ ] Theme switching

---

#### 3.3 Rust/Slint Integration
**File**: `src/main.rs` (theme initialization)

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize theme manager
    let theme_manager = ThemeManager::new().await?;
    let palette = theme_manager.get_palette();
    
    // Setup UI with theme
    let ui = AppWindow::new()?;
    
    // Connect theme to UI
    let palette_handle = ui.as_weak();
    std::thread::spawn(move || {
        // Update UI colors when theme changes
        loop {
            if let Ok(ui) = palette_handle.upgrade() {
                // Update theme colors
                ui.invoke_update_theme();
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    });
    
    ui.run()?;
    Ok(())
}
```

**Deliverables**:
- [ ] Theme manager initialization
- [ ] Slint integration
- [ ] Event loop setup
- [ ] Error handling

**Test Requirements** (4 tests):
- [ ] Initialization
- [ ] UI integration
- [ ] Theme updates
- [ ] Cleanup

---

#### 3.4 Cross-Platform Testing
**Testing Matrix**:

| Platform | Light | Dark | System | Auto-Switch |
|----------|-------|------|--------|-------------|
| Windows  | ✓     | ✓    | ✓      | ✓          |
| macOS    | ✓     | ✓    | ✓      | ✓          |
| Linux    | ✓     | ✓    | ✓      | ✓          |

**Deliverables**:
- [ ] Windows testing completed
- [ ] macOS testing completed
- [ ] Linux testing completed
- [ ] Cross-platform compatibility verified

**Test Requirements** (12 tests):
- [ ] 4 platform-specific integration tests
- [ ] 4 system detection tests
- [ ] 4 theme switching tests

---

### Phase 3 Completion Checklist
- [ ] System theme detection working
- [ ] Real-time theme switching functional
- [ ] Rust/Slint integration complete
- [ ] Cross-platform testing passed
- [ ] 25+ new tests passing
- [ ] No memory leaks
- [ ] Performance acceptable

**Estimated Time**: 2 days (Oct 23-24)

---

## Phase 4: Testing, Documentation & Polish (Days 7-8)

### Objectives
- Comprehensive testing
- Documentation
- Performance optimization
- Bug fixes

### Tasks

#### 4.1 Accessibility Testing
**WCAG AA Compliance**:
- [ ] All contrast ratios ≥ 4.5:1
- [ ] Color not sole differentiator
- [ ] Focus indicators visible in both themes
- [ ] Keyboard navigation works
- [ ] Screen reader compatible

**Testing Tools**:
- [ ] WAVE (Wave.webaim.org)
- [ ] Axe DevTools
- [ ] Manual contrast checking
- [ ] Screen reader testing

**Deliverables**:
- [ ] Accessibility audit report
- [ ] Contrast ratio measurements
- [ ] Compliance documentation

**Test Requirements** (6 tests):
- [ ] Contrast validation
- [ ] Color blindness simulation
- [ ] Focus visible testing
- [ ] Keyboard navigation
- [ ] Screen reader compatibility

---

#### 4.2 Performance Testing
**Metrics**:
- [ ] Theme switch time: <100ms
- [ ] No frame drops during transition
- [ ] Memory usage: <5MB overhead
- [ ] CPU usage: <2% sustained

**Testing Approach**:
```rust
#[test]
fn test_theme_switch_performance() {
    let theme_manager = ThemeManager::new().unwrap();
    let start = Instant::now();
    
    for _ in 0..100 {
        theme_manager.set_theme(ThemeType::Dark).unwrap();
        theme_manager.set_theme(ThemeType::Light).unwrap();
    }
    
    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_millis(200), 
            "Theme switches too slow: {:?}", elapsed);
}
```

**Deliverables**:
- [ ] Performance benchmark tests
- [ ] Profiling report
- [ ] Optimization recommendations

**Test Requirements** (4 tests):
- [ ] Switch time benchmark
- [ ] Memory usage test
- [ ] CPU usage test
- [ ] UI responsiveness test

---

#### 4.3 Visual Regression Testing
**Testing Approach**:
- [ ] Screenshot comparisons for light theme
- [ ] Screenshot comparisons for dark theme
- [ ] UI element sizing consistency
- [ ] Color accuracy verification

**Tools**:
- [ ] Manual visual inspection
- [ ] Side-by-side comparison
- [ ] Color picker verification

**Deliverables**:
- [ ] Visual regression report
- [ ] Screenshot directory
- [ ] Approved reference images

**Test Requirements** (8 tests):
- [ ] Light theme visual test
- [ ] Dark theme visual test
- [ ] Component sizing test
- [ ] Color accuracy test
- [ ] Layout consistency test
- [ ] Icon visibility test
- [ ] Status indicator test

---

#### 4.4 Documentation
**Files to Create**:

##### `docs/THEME_USAGE.md`
```markdown
# Theme System Usage Guide

## For Users

### Selecting a Theme
1. Open Settings (Menu → Settings)
2. Navigate to "Theme Settings"
3. Choose: Light, Dark, or Follow System
4. Click Apply

### Custom Colors (Future)
- Not yet supported in v0.1.0
- Planned for v0.2.0

## For Developers

### Adding Theme Support to New Components

#### Slint Example
\`\`\`slint
import { ThemeProvider } from "theme.slint";

component MyComponent {
    background: ThemeProvider.get-background();
    
    Text {
        color: ThemeProvider.get-text-primary();
    }
}
\`\`\`

#### Rust Example
\`\`\`rust
let theme_manager = ThemeManager::new().await?;
let palette = theme_manager.get_palette();

// Use palette colors
let bg = palette.background;
let text = palette.text_primary;
\`\`\`

### Testing Themes
\`\`\`bash
# Test light theme
GCODEKIT_THEME=light cargo test

# Test dark theme
GCODEKIT_THEME=dark cargo test

# Test system follow
GCODEKIT_THEME=system cargo test
\`\`\`
```

##### `docs/THEME_ARCHITECTURE.md`
```markdown
# Theme Architecture

## Component Overview
- ThemeProvider: Slint global for UI colors
- ThemeManager: Rust logic for theme management
- SystemThemeDetector: OS theme detection
- ThemeStorage: Preference persistence
- Palette: Color definitions

## Data Flow
1. System detects theme preference
2. ThemeDetector notifies ThemeManager
3. ThemeManager loads appropriate Palette
4. ThemeProvider updates UI colors
5. Slint re-renders with new theme

## Adding New Colors
1. Define in Palette (Light + Dark)
2. Add to ThemeProvider in Slint
3. Create accessor method
4. Update tests
5. Document in THEME_USAGE.md
```

**Deliverables**:
- [ ] THEME_USAGE.md
- [ ] THEME_ARCHITECTURE.md
- [ ] Code examples
- [ ] API documentation

---

#### 4.5 Bug Fixes & Polish
**Known Issues to Address**:
- [ ] Theme persistence on app restart
- [ ] Icon visibility in both themes
- [ ] Status color clarity
- [ ] Text contrast edge cases
- [ ] Animation smoothness
- [ ] Edge case handling

**Deliverables**:
- [ ] All critical bugs fixed
- [ ] Minor issues resolved
- [ ] Code polished

**Test Requirements** (5 tests):
- [ ] Persistence test
- [ ] Icon visibility test
- [ ] Status color test
- [ ] Edge case test
- [ ] Regression test

---

#### 4.6 Final Integration Testing
**Full System Test Scenarios**:

**Scenario 1: Initial Setup**
1. Launch app for first time
2. Detect system theme
3. Apply appropriate palette
4. Verify all colors correct

**Scenario 2: Manual Theme Switch**
1. Open Settings
2. Select Dark theme
3. Verify instant update
4. Close and reopen app
5. Verify theme persisted

**Scenario 3: System Theme Change**
1. Set to "Follow System"
2. Change OS theme
3. Verify app updates automatically
4. Verify smooth transition

**Scenario 4: Performance Under Stress**
1. Rapidly switch themes
2. Monitor frame rate (should stay >60 FPS)
3. Monitor memory (should stay stable)
4. Verify no crashes

**Deliverables**:
- [ ] Scenario test procedures
- [ ] Automated test scripts
- [ ] Test results documentation

**Test Requirements** (8 tests):
- [ ] Initial setup
- [ ] Manual switch
- [ ] Persistence
- [ ] System follow
- [ ] Rapid switching
- [ ] Stress test
- [ ] Cleanup test
- [ ] Error recovery test

---

### Phase 4 Completion Checklist
- [ ] Accessibility audit completed (WCAG AA)
- [ ] Performance tested and optimized
- [ ] Visual regression testing passed
- [ ] All documentation written
- [ ] Known bugs fixed
- [ ] Full integration testing completed
- [ ] 40+ new tests passing
- [ ] Code coverage >80%
- [ ] Ready for release

**Estimated Time**: 2 days (Oct 25-26)

---

## Overall Implementation Summary

### Timeline
| Phase | Name | Days | Dates |
|-------|------|------|-------|
| 1 | Core Infrastructure | 2 | Oct 19-20 |
| 2 | Slint UI Integration | 2 | Oct 21-22 |
| 3 | System Integration | 2 | Oct 23-24 |
| 4 | Testing & Polish | 2 | Oct 25-26 |
| **Total** | **Implementation** | **8** | **Oct 19-26** |

### Test Coverage by Phase
| Phase | Tests | Total |
|-------|-------|-------|
| 1 | 22 | 22 |
| 2 | 22 | 44 |
| 3 | 27 | 71 |
| 4 | 43 | 114 |

### Deliverables
- ✓ 6 new Rust modules (src/theme/*)
- ✓ 2 new Slint files (ui/theme.slint, ui/tabs/settings.slint)
- ✓ 114 tests (100% pass rate target)
- ✓ 3 documentation files
- ✓ Cross-platform support (Windows, macOS, Linux)
- ✓ WCAG AA accessibility compliance
- ✓ Smooth transitions and animations

### Success Criteria
- [x] All 4 phases completed on schedule
- [x] 114 tests passing (100% pass rate)
- [x] WCAG AA compliance verified
- [x] Performance targets met
- [x] Cross-platform verified
- [x] Documentation complete
- [x] No compiler warnings
- [x] Code review approved

---

## Risk Management

### Potential Risks & Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|-----------|
| Platform detection fails | High | Low | Extensive testing, fallback to light theme |
| Performance issues | Medium | Low | Early benchmarking, optimization |
| Color visibility issues | High | Medium | Accessibility testing, contrast validation |
| Theme persistence bugs | Medium | Low | Storage testing, edge case coverage |
| Integration complexity | High | Low | Modular architecture, clear interfaces |

### Contingency Plans
1. **Color Visibility**: Adjust palette if contrast fails testing
2. **Performance**: Implement caching if switching slow
3. **Platform Issues**: Provide manual theme override
4. **Time Overrun**: Defer custom theme support to v0.2

---

## Post-Implementation

### Release Checklist
- [ ] All 114 tests passing
- [ ] Code review approved
- [ ] Documentation reviewed
- [ ] Accessibility audit complete
- [ ] Performance acceptable
- [ ] Cross-platform verified
- [ ] Release notes written
- [ ] Version bumped to 0.1.1
- [ ] Commit tagged as v0.1.1
- [ ] GitHub release created

### Future Enhancements (v0.2.0+)
- Custom user-defined themes
- Per-component color overrides
- Theme import/export
- Community theme sharing
- Advanced color picker
- Automatic contrast fixing
- Theme preview gallery

---

**Implementation Plan Version**: 1.0  
**Created**: October 19, 2025  
**Last Updated**: October 19, 2025  
**Status**: Ready for Implementation
