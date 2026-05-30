use std::collections::HashMap;

/// The seven cultural mathematical traditions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Tradition {
    Western,
    Chinese,
    Vedic,
    Islamic,
    Japanese,
    African,
    Indigenous,
}

impl Tradition {
    pub fn all() -> Vec<Tradition> {
        vec![
            Tradition::Western,
            Tradition::Chinese,
            Tradition::Vedic,
            Tradition::Islamic,
            Tradition::Japanese,
            Tradition::African,
            Tradition::Indigenous,
        ]
    }

    /// Human-readable label for the tradition.
    pub fn label(&self) -> &str {
        match self {
            Tradition::Western => "Western",
            Tradition::Chinese => "Chinese",
            Tradition::Vedic => "Vedic",
            Tradition::Islamic => "Islamic",
            Tradition::Japanese => "Japanese",
            Tradition::African => "African",
            Tradition::Indigenous => "Indigenous",
        }
    }
}

/// A mathematical concept as understood across multiple traditions.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MathematicalConcept {
    /// Canonical english-key name for this concept.
    pub name: String,
    /// The name preferred in Western mathematical writing.
    pub western_name: String,
    /// Map from tradition → the term used in that tradition.
    pub tradition_names: HashMap<Tradition, String>,
    /// Formal mathematical definition (Western-style).
    pub formal_definition: String,
    /// Intuitive, cross-cultural description.
    pub intuitive_description: String,
}

impl MathematicalConcept {
    /// Return the name for a given tradition, falling back to the western name.
    pub fn name_in(&self, tradition: &Tradition) -> &str {
        self.tradition_names
            .get(tradition)
            .map(|s| s.as_str())
            .unwrap_or(&self.western_name)
    }

    /// Register (or overwrite) the name for a tradition.
    pub fn register_name(&mut self, tradition: Tradition, name: String) {
        self.tradition_names.insert(tradition, name);
    }
}

// ---------------------------------------------------------------------------
// ConceptLibrary
// ---------------------------------------------------------------------------

/// A library of cross-cultural mathematical concepts.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConceptLibrary {
    pub concepts: HashMap<String, MathematicalConcept>,
}

/// Helper macro to build a [`HashMap<Tradition, String>`] inline.
#[macro_export]
macro_rules! tradition_map {
    ( $( $tradition:ident : $value:expr ),+ $(,)? ) => {
        {
            use $crate::Tradition;
            let mut m = std::collections::HashMap::new();
            $(
                m.insert(Tradition::$tradition, $value.into());
            )+
            m
        }
    };
}

impl ConceptLibrary {
    /// Look up a concept by its canonical key.
    pub fn get(&self, concept: &str) -> Option<&MathematicalConcept> {
        self.concepts.get(concept)
    }

    /// Register a new concept into the library.
    pub fn register(&mut self, concept: MathematicalConcept) {
        self.concepts.insert(concept.name.clone(), concept);
    }

    /// Return all concepts as seen through the lens of a single tradition.
    /// Returns `(canonical_key, local_name)` pairs.
    pub fn tradition_view(&self, tradition: &Tradition) -> Vec<(&str, &str)> {
        let mut result: Vec<(&str, &str)> = self
            .concepts
            .values()
            .map(|c| {
                let local = c.name_in(tradition);
                (c.name.as_str(), local)
            })
            .collect();
        result.sort_by(|a, b| a.0.cmp(b.0));
        result
    }

    /// Given a localised name (in any tradition), find every tradition that
    /// uses that name and the concept key it maps to.
    pub fn find_equivalents(&self, name: &str) -> Vec<(&Tradition, &str)> {
        let mut results = Vec::new();
        for concept in self.concepts.values() {
            for (tradition, tname) in &concept.tradition_names {
                if tname == name {
                    results.push((tradition, concept.name.as_str()));
                }
            }
            // Also check the western_name field
            if concept.western_name == name {
                results.push((&Tradition::Western, concept.name.as_str()));
            }
        }
        results
    }

    /// Build a library pre-populated with the core cross-cultural concepts.
    pub fn preloaded() -> Self {
        let mut library = ConceptLibrary {
            concepts: HashMap::new(),
        };

        // --- conservation --------------------------------------------------
        library.register(MathematicalConcept {
            name: "conservation".into(),
            western_name: "invariant".into(),
            tradition_names: tradition_map! {
                Western: "invariant",
                Chinese: "tao balance (道平衡)",
                Vedic: "ṛta (ऋत)",
                Islamic: "al-jabr balance (الجبر)",
                Japanese: "hózhó harmony",
                African: "ubuntu reciprocity",
                Indigenous: "seventh generation",
            },
            formal_definition: "A quantity or property that remains unchanged under a specified set of transformations.".into(),
            intuitive_description: "What stays the same even when everything else changes.".into(),
        });

        // --- zero ----------------------------------------------------------
        library.register(MathematicalConcept {
            name: "zero".into(),
            western_name: "additive identity".into(),
            tradition_names: tradition_map! {
                Western: "additive identity",
                Chinese: "wuji (无极)",
                Vedic: "śūnya (शून्य)",
                Islamic: "ṣifr (صفر)",
                Japanese: "mu (無)",
                African: "the void between",
                Indigenous: "the space that holds",
            },
            formal_definition: "The additive identity element 0 such that a + 0 = a for all a.".into(),
            intuitive_description: "Not nothing — the potential from which all things arise.".into(),
        });

        // --- symmetry ------------------------------------------------------
        library.register(MathematicalConcept {
            name: "symmetry".into(),
            western_name: "invariant under transformation".into(),
            tradition_names: tradition_map! {
                Western: "invariant under transformation",
                Chinese: "taiji balance (太极平衡)",
                Vedic: "sāmarasya (सामरस्य)",
                Islamic: "geometric pattern (نمط هندسي)",
                Japanese: "ma (間)",
                African: "pattern repeat",
                Indigenous: "kinship mirror",
            },
            formal_definition: "A transformation that leaves the object unchanged.".into(),
            intuitive_description: "The harmony of a pattern that echoes through scale and perspective.".into(),
        });

        // --- consensus -----------------------------------------------------
        library.register(MathematicalConcept {
            name: "consensus".into(),
            western_name: "agreement protocol".into(),
            tradition_names: tradition_map! {
                Western: "agreement protocol",
                Chinese: "wu-wei harmony (无为和谐)",
                Vedic: "sangha (संघ)",
                Islamic: "shura (شورى)",
                Japanese: "wa (和)",
                African: "palaver tree",
                Indigenous: "council circle",
            },
            formal_definition: "A process by which distributed agents reach a shared state.".into(),
            intuitive_description: "Coming together not by force but by finding the common thread.".into(),
        });

        // --- inheritance ---------------------------------------------------
        library.register(MathematicalConcept {
            name: "inheritance".into(),
            western_name: "knowledge transfer".into(),
            tradition_names: tradition_map! {
                Western: "knowledge transfer",
                Chinese: "dao lineage (道统)",
                Vedic: "guru-shishya (गुरु-शिष्य)",
                Islamic: "isnad chain (إسناد)",
                Japanese: "iemoto system (家元)",
                African: "griot tradition",
                Indigenous: "songline",
            },
            formal_definition: "The mechanism by which properties or knowledge are passed from one entity to its descendants.".into(),
            intuitive_description: "What the ancestors carry forward into the hands of the next generation.".into(),
        });

        library
    }
}

// ---------------------------------------------------------------------------
// Translation
// ---------------------------------------------------------------------------

/// The result of translating one concept from one tradition to another.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranslationResult {
    pub from_tradition: Tradition,
    pub to_tradition: Tradition,
    pub concept: String,
    pub from_name: String,
    pub to_name: String,
    pub confidence: f64,
    pub notes: String,
}

/// Translates mathematical concepts between cultural traditions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraditionTranslator {
    pub library: ConceptLibrary,
}

impl TraditionTranslator {
    pub fn new(library: ConceptLibrary) -> Self {
        TraditionTranslator { library }
    }

    /// Translate a concept from one tradition to another.
    pub fn translate(
        &self,
        concept: &str,
        from: &Tradition,
        to: &Tradition,
    ) -> Option<TranslationResult> {
        let c = self.library.get(concept)?;
        let from_name = c.name_in(from).to_string();
        let to_name = c.name_in(to).to_string();

        let same = from == to;
        let matching = from_name == to_name;
        let confidence = if same {
            1.0
        } else if matching {
            0.9
        } else {
            0.85
        };

        let notes = if same {
            format!("No translation needed — both are {}.", from.label())
        } else if matching {
            format!(
                "The term '{}' is shared between {} and {}.",
                from_name,
                from.label(),
                to.label()
            )
        } else {
            format!(
                "'{}' ({}) → '{}' ({}).",
                from_name,
                from.label(),
                to_name,
                to.label()
            )
        };

        Some(TranslationResult {
            from_tradition: from.clone(),
            to_tradition: to.clone(),
            concept: concept.to_string(),
            from_name,
            to_name,
            confidence,
            notes,
        })
    }

    /// Translate a concept to *all* other traditions.
    pub fn translate_all(&self, concept: &str) -> Vec<TranslationResult> {
        let mut results = Vec::new();
        let Some(c) = self.library.get(concept) else {
            return results;
        };

        for tradition in Tradition::all() {
            for other in Tradition::all() {
                if tradition != other {
                    let from_name = c.name_in(&tradition).to_string();
                    let to_name = c.name_in(&other).to_string();
                    let matching = from_name == to_name;
                    let notes = format!(
                        "'{}' ({}) → '{}' ({}).",
                        c.name_in(&tradition),
                        tradition.label(),
                        c.name_in(&other),
                        other.label()
                    );
                    results.push(TranslationResult {
                        from_tradition: tradition.clone(),
                        to_tradition: other,
                        concept: concept.to_string(),
                        from_name,
                        to_name,
                        confidence: if matching { 0.9 } else { 0.85 },
                        notes,
                    });
                }
            }
        }

        results
    }

    /// Get an explanatory string for a concept in a given tradition's terms.
    pub fn explain_in(&self, concept: &str, tradition: &Tradition) -> Option<String> {
        let c = self.library.get(concept)?;
        let local_name = c.name_in(tradition);
        Some(format!(
            "In the {} tradition, '{}' is known as '{}'. Intuitively: {}",
            tradition.label(),
            concept,
            local_name,
            c.intuitive_description
        ))
    }
}

// ---------------------------------------------------------------------------
// PolyglotRoom
// ---------------------------------------------------------------------------

/// A virtual room where multiple cultural traditions coexist around a
/// dominant tradition, and concept expressions are computed collectively.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolyglotRoom {
    pub room_id: String,
    pub dominant_tradition: Tradition,
    pub available_traditions: Vec<Tradition>,
    pub concept_map: HashMap<String, MathematicalConcept>,
    pub tick: u64,
}

impl PolyglotRoom {
    pub fn new(room_id: &str, dominant: Tradition) -> Self {
        PolyglotRoom {
            room_id: room_id.to_string(),
            dominant_tradition: dominant.clone(),
            available_traditions: vec![dominant],
            concept_map: HashMap::new(),
            tick: 0,
        }
    }

    /// Add a tradition to this room (deduplicated).
    pub fn add_tradition(&mut self, tradition: Tradition) {
        if !self.available_traditions.contains(&tradition) {
            self.available_traditions.push(tradition);
        }
    }

    /// Express a concept in *all* available traditions.
    /// Returns a map from tradition → the local name.
    pub fn express(&self, concept: &str) -> HashMap<Tradition, String> {
        let mut map = HashMap::new();
        if let Some(c) = self.concept_map.get(concept) {
            for tradition in &self.available_traditions {
                map.insert(tradition.clone(), c.name_in(tradition).to_string());
            }
        }
        map
    }

    /// Change the dominant tradition.
    pub fn switch_dominant(&mut self, tradition: Tradition) {
        self.dominant_tradition = tradition;
        self.tick += 1;
    }

    /// A human-readable summary of the room.
    pub fn room_summary(&self) -> String {
        let traditions: Vec<&str> = self
            .available_traditions
            .iter()
            .map(|t| t.label())
            .collect();

        let concepts: Vec<&str> = self.concept_map.keys().map(|s| s.as_str()).collect();

        format!(
            "Room '{}' (tick {}) | Dominant: {} | Traditions: [{}] | Concepts: [{}]",
            self.room_id,
            self.tick,
            self.dominant_tradition.label(),
            traditions.join(", "),
            concepts.join(", ")
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- MathematicalConcept tests ---------------------------------------

    #[test]
    fn test_name_in_western_fallback() {
        let mut names = HashMap::new();
        names.insert(Tradition::Chinese, "tao".into());
        let c = MathematicalConcept {
            name: "conservation".into(),
            western_name: "invariant".into(),
            tradition_names: names,
            formal_definition: "def".into(),
            intuitive_description: "desc".into(),
        };
        assert_eq!(c.name_in(&Tradition::Western), "invariant");
        assert_eq!(c.name_in(&Tradition::Chinese), "tao");
        // Falls back to western name for unknown tradition
        assert_eq!(c.name_in(&Tradition::Vedic), "invariant");
    }

    #[test]
    fn test_register_name_overwrites() {
        let mut c = MathematicalConcept {
            name: "zero".into(),
            western_name: "zero".into(),
            tradition_names: HashMap::new(),
            formal_definition: "def".into(),
            intuitive_description: "desc".into(),
        };
        c.register_name(Tradition::Japanese, "rei".into());
        assert_eq!(c.name_in(&Tradition::Japanese), "rei");
        c.register_name(Tradition::Japanese, "mu".into());
        assert_eq!(c.name_in(&Tradition::Japanese), "mu");
    }

    // ---- ConceptLibrary tests --------------------------------------------

    #[test]
    fn test_preloaded_has_all_keys() {
        let lib = ConceptLibrary::preloaded();
        assert!(lib.get("conservation").is_some());
        assert!(lib.get("zero").is_some());
        assert!(lib.get("symmetry").is_some());
        assert!(lib.get("consensus").is_some());
        assert!(lib.get("inheritance").is_some());
        assert!(lib.get("nonexistent").is_none());
    }

    #[test]
    fn test_register_new_concept() {
        let mut lib = ConceptLibrary::preloaded();
        let c = MathematicalConcept {
            name: "infinity".into(),
            western_name: "limitless".into(),
            tradition_names: tradition_map! {
                Chinese: "wu ji (无极)",
                Vedic: "ananta (अनन्त)",
            },
            formal_definition: "A quantity larger than any finite value.".into(),
            intuitive_description: "The unending horizon.".into(),
        };
        lib.register(c);
        assert!(lib.get("infinity").is_some());
    }

    #[test]
    fn test_tradition_view() {
        let lib = ConceptLibrary::preloaded();
        let view = lib.tradition_view(&Tradition::Chinese);
        // Should have 5 entries, sorted by key
        assert_eq!(view.len(), 5);
        assert_eq!(view[0].0, "consensus");
        assert!(view[0].1.contains("wu-wei"));
    }

    #[test]
    fn test_find_equivalents_hit() {
        let lib = ConceptLibrary::preloaded();
        let equiv = lib.find_equivalents("mu (無)");
        assert!(!equiv.is_empty());
        let has_japanese_zero = equiv
            .iter()
            .any(|(t, k)| **t == Tradition::Japanese && *k == "zero");
        assert!(has_japanese_zero);
    }

    #[test]
    fn test_find_equivalents_miss() {
        let lib = ConceptLibrary::preloaded();
        let equiv = lib.find_equivalents("nobody-calls-it-this");
        assert!(equiv.is_empty());
    }

    // ---- TraditionTranslator tests ---------------------------------------

    #[test]
    fn test_translate_same_tradition() {
        let lib = ConceptLibrary::preloaded();
        let translator = TraditionTranslator::new(lib);
        let result = translator
            .translate("zero", &Tradition::Western, &Tradition::Western)
            .unwrap();
        assert_eq!(result.confidence, 1.0);
        assert_eq!(result.from_name, result.to_name);
    }

    #[test]
    fn test_translate_cross_tradition() {
        let lib = ConceptLibrary::preloaded();
        let translator = TraditionTranslator::new(lib);
        let result = translator
            .translate("conservation", &Tradition::Western, &Tradition::Chinese)
            .unwrap();
        assert_eq!(result.concept, "conservation");
        assert_eq!(result.from_name, "invariant");
        assert!(result.to_name.contains("tao"));
        assert_eq!(result.confidence, 0.85);
    }

    #[test]
    fn test_translate_unknown_concept() {
        let lib = ConceptLibrary::preloaded();
        let translator = TraditionTranslator::new(lib);
        assert!(translator.translate("foo", &Tradition::Western, &Tradition::Chinese).is_none());
    }

    #[test]
    fn test_translate_all_count() {
        let lib = ConceptLibrary::preloaded();
        let translator = TraditionTranslator::new(lib);
        let results = translator.translate_all("zero");
        // 7 traditions → 7*6 = 42 directed translations
        assert_eq!(results.len(), 42);
    }

    #[test]
    fn test_translate_all_unknown() {
        let lib = ConceptLibrary::preloaded();
        let translator = TraditionTranslator::new(lib);
        let results = translator.translate_all("void");
        assert!(results.is_empty());
    }

    #[test]
    fn test_explain_in() {
        let lib = ConceptLibrary::preloaded();
        let translator = TraditionTranslator::new(lib);
        let explanation = translator
            .explain_in("symmetry", &Tradition::Japanese)
            .unwrap();
        assert!(explanation.contains("ma (間)"));
        assert!(explanation.contains("Japanese"));
        assert!(explanation.contains("harmony"));
    }

    #[test]
    fn test_explain_in_unknown() {
        let lib = ConceptLibrary::preloaded();
        let translator = TraditionTranslator::new(lib);
        assert!(translator.explain_in("bogus", &Tradition::Western).is_none());
    }

    // ---- PolyglotRoom tests ----------------------------------------------

    #[test]
    fn test_room_new() {
        let room = PolyglotRoom::new("test-room", Tradition::Vedic);
        assert_eq!(room.room_id, "test-room");
        assert_eq!(room.dominant_tradition, Tradition::Vedic);
        assert_eq!(room.available_traditions.len(), 1);
        assert_eq!(room.tick, 0);
    }

    #[test]
    fn test_room_add_tradition() {
        let mut room = PolyglotRoom::new("r1", Tradition::Western);
        room.add_tradition(Tradition::Chinese);
        room.add_tradition(Tradition::Chinese); // no duplicate
        assert_eq!(room.available_traditions.len(), 2);
    }

    #[test]
    fn test_room_express() {
        let mut room = PolyglotRoom::new("r2", Tradition::Western);
        room.add_tradition(Tradition::Chinese);

        let lib = ConceptLibrary::preloaded();
        let conservation = lib.get("conservation").unwrap().clone();
        room.concept_map
            .insert("conservation".into(), conservation);

        let expr = room.express("conservation");
        assert!(expr.contains_key(&Tradition::Western));
        assert!(expr.contains_key(&Tradition::Chinese));
        assert_eq!(expr.get(&Tradition::Western).unwrap(), "invariant");
    }

    #[test]
    fn test_room_express_unknown() {
        let room = PolyglotRoom::new("r3", Tradition::Western);
        let expr = room.express("nothing");
        assert!(expr.is_empty());
    }

    #[test]
    fn test_room_switch_dominant() {
        let mut room = PolyglotRoom::new("r4", Tradition::Western);
        assert_eq!(room.tick, 0);
        room.switch_dominant(Tradition::Islamic);
        assert_eq!(room.dominant_tradition, Tradition::Islamic);
        assert_eq!(room.tick, 1);
    }

    #[test]
    fn test_room_summary() {
        let mut room = PolyglotRoom::new("hall", Tradition::African);
        room.add_tradition(Tradition::Indigenous);

        let lib = ConceptLibrary::preloaded();
        room.concept_map
            .insert("zero".into(), lib.get("zero").unwrap().clone());

        let summary = room.room_summary();
        assert!(summary.contains("hall"));
        assert!(summary.contains("African"));
        assert!(summary.contains("Indigenous"));
        assert!(summary.contains("zero"));
    }

    // ---- tradition_map! macro test ---------------------------------------

    #[test]
    fn test_tradition_map_macro() {
        use std::collections::HashMap;
        let map: HashMap<Tradition, String> = tradition_map! {
            Western: "hello",
            Vedic: "namaste",
            Japanese: "konnichiwa",
        };
        assert_eq!(map.get(&Tradition::Western).unwrap(), "hello");
        assert_eq!(map.get(&Tradition::Vedic).unwrap(), "namaste");
        assert_eq!(map.get(&Tradition::Japanese).unwrap(), "konnichiwa");
        assert_eq!(map.len(), 3);
    }

    // ---- Serde round-trip tests ------------------------------------------

    #[test]
    fn test_tradition_serde_json() {
        let t = Tradition::Indigenous;
        let json = serde_json::to_string(&t).unwrap();
        let back: Tradition = serde_json::from_str(&json).unwrap();
        assert_eq!(back, Tradition::Indigenous);
    }

    #[test]
    fn test_concept_serde_json() {
        let c = MathematicalConcept {
            name: "conservation".into(),
            western_name: "invariant".into(),
            tradition_names: tradition_map! {
                Chinese: "tao balance",
            },
            formal_definition: "def".into(),
            intuitive_description: "desc".into(),
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: MathematicalConcept = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, "conservation");
        assert_eq!(back.name_in(&Tradition::Chinese), "tao balance");
    }

    #[test]
    fn test_library_serde_json() {
        let lib = ConceptLibrary::preloaded();
        let json = serde_json::to_string(&lib).unwrap();
        let back: ConceptLibrary = serde_json::from_str(&json).unwrap();
        assert!(back.get("zero").is_some());
        assert_eq!(back.get("zero").unwrap().western_name, "additive identity");
    }

    #[test]
    fn test_room_serde_json() {
        let mut room = PolyglotRoom::new("hall", Tradition::Islamic);
        room.add_tradition(Tradition::African);
        room.add_tradition(Tradition::Japanese);
        let json = serde_json::to_string(&room).unwrap();
        let back: PolyglotRoom = serde_json::from_str(&json).unwrap();
        assert_eq!(back.room_id, "hall");
        assert_eq!(back.available_traditions.len(), 3);
    }

    #[test]
    fn test_translation_result_serde_json() {
        let result = TranslationResult {
            from_tradition: Tradition::Western,
            to_tradition: Tradition::Chinese,
            concept: "zero".into(),
            from_name: "additive identity".into(),
            to_name: "wuji (无极)".into(),
            confidence: 0.85,
            notes: "test".into(),
        };
        let json = serde_json::to_string(&result).unwrap();
        let back: TranslationResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.confidence, 0.85);
        assert_eq!(back.to_name, "wuji (无极)");
    }

    // ---- Tradition::all() and label --------------------------------------

    #[test]
    fn test_tradition_all_contains_all_seven() {
        let all = Tradition::all();
        assert_eq!(all.len(), 7);
        assert!(all.contains(&Tradition::Western));
        assert!(all.contains(&Tradition::Indigenous));
    }

    #[test]
    fn test_tradition_label() {
        assert_eq!(Tradition::Western.label(), "Western");
        assert_eq!(Tradition::Indigenous.label(), "Indigenous");
    }

    // ---- TraditionTranslator with matching names (shared term) -----------

    #[test]
    fn test_translate_matching_names_high_confidence() {
        let mut lib = ConceptLibrary::preloaded();
        // Force same name for Western and Chinese
        let mut c = lib.get("zero").unwrap().clone();
        c.register_name(Tradition::Chinese, "additive identity".into());
        lib.register(c);

        let translator = TraditionTranslator::new(lib);
        let result = translator
            .translate("zero", &Tradition::Western, &Tradition::Chinese)
            .unwrap();
        assert_eq!(result.confidence, 0.9);
    }

    // ---- PolyglotRoom express without concept_map entry ------------------

    #[test]
    fn test_room_express_no_concept_registered() {
        let mut room = PolyglotRoom::new("empty", Tradition::Western);
        room.add_tradition(Tradition::Chinese);
        let expr = room.express("symmetry");
        assert!(expr.is_empty());
    }

    // ---- tradition_view empty library ------------------------------------

    #[test]
    fn test_tradition_view_empty() {
        let lib = ConceptLibrary {
            concepts: HashMap::new(),
        };
        let view = lib.tradition_view(&Tradition::Western);
        assert!(view.is_empty());
    }

    // ---- find_equivalents on western_name --------------------------------

    #[test]
    fn test_find_equivalents_western_name() {
        let lib = ConceptLibrary::preloaded();
        let equiv = lib.find_equivalents("invariant");
        assert!(!equiv.is_empty());
        assert!(equiv.iter().any(|(t, _)| **t == Tradition::Western));
    }

    // ---- PolyglotRoom concept_map register -------------------------------

    #[test]
    fn test_room_express_multiple_concepts() {
        let mut room = PolyglotRoom::new("multi", Tradition::Western);
        room.add_tradition(Tradition::Vedic);

        let lib = ConceptLibrary::preloaded();
        room.concept_map
            .insert("zero".into(), lib.get("zero").unwrap().clone());
        room.concept_map
            .insert("symmetry".into(), lib.get("symmetry").unwrap().clone());

        let zero_expr = room.express("zero");
        assert_eq!(zero_expr.get(&Tradition::Western).unwrap(), "additive identity");

        let sym_expr = room.express("symmetry");
        assert!(sym_expr.get(&Tradition::Vedic).unwrap().contains("sāmarasya"));
    }

    // ---- TraditionTranslator explain_in Vedic variant ----------------------

    #[test]
    fn test_explain_in_vedic() {
        let lib = ConceptLibrary::preloaded();
        let translator = TraditionTranslator::new(lib);
        let explanation = translator
            .explain_in("inheritance", &Tradition::Vedic)
            .unwrap();
        assert!(explanation.contains("guru-shishya"));
        assert!(explanation.contains("Vedic"));
    }

    // ---- PolyglotRoom summary after switch -------------------------------

    #[test]
    fn test_room_summary_after_switch() {
        let mut room = PolyglotRoom::new("switch-test", Tradition::Western);
        room.add_tradition(Tradition::Japanese);
        room.add_tradition(Tradition::Islamic);

        let lib = ConceptLibrary::preloaded();
        room.concept_map
            .insert("consensus".into(), lib.get("consensus").unwrap().clone());

        room.switch_dominant(Tradition::Japanese);

        let summary = room.room_summary();
        assert!(summary.contains("Japanese"));
        assert!(summary.contains("tick 1"));
        assert!(summary.contains("consensus"));
        assert!(summary.contains("Western"));
        assert!(summary.contains("Islamic"));
    }
}
