// Imports
use gtk4::glib;
use rnote_engine::WidgetFlags;
use rnote_engine::engine::EngineConfig;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, glib::Boxed)]
#[boxed_type(name = "WidgetFlagsBoxed")]
pub(crate) struct WidgetFlagsBoxed(WidgetFlags);

impl From<WidgetFlags> for WidgetFlagsBoxed {
    fn from(value: WidgetFlags) -> Self {
        Self(value)
    }
}

impl From<WidgetFlagsBoxed> for WidgetFlags {
    fn from(WidgetFlagsBoxed(value): WidgetFlagsBoxed) -> Self {
        value
    }
}

impl WidgetFlagsBoxed {
    pub(crate) fn inner(self) -> WidgetFlags {
        self.0
    }
}

#[derive(Debug, Clone, glib::Boxed)]
#[boxed_type(name = "EngineConfigBoxed")]
pub(crate) struct EngineConfigBoxed(EngineConfig);

impl From<EngineConfig> for EngineConfigBoxed {
    fn from(value: EngineConfig) -> Self {
        Self(value)
    }
}

impl Default for EngineConfigBoxed {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl From<EngineConfigBoxed> for EngineConfig {
    fn from(EngineConfigBoxed(value): EngineConfigBoxed) -> Self {
        value
    }
}

impl EngineConfigBoxed {
    pub(crate) fn inner(self) -> EngineConfig {
        self.0
    }
}
