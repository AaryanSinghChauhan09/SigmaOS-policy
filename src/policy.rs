#[derive(Debug, Clone, PartialEq)]
pub enum PolicyDecision {
    Allow,
    Deny,
    Challenge,
}

#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub action: String,
    pub subject: String,
    pub decision: PolicyDecision,
}

impl PolicyRule {
    pub fn new(action: &str, subject: &str, decision: PolicyDecision) -> Self {
        Self {
            action: action.to_string(),
            subject: subject.to_string(),
            decision,
        }
    }
}

pub struct PolicyEngine {
    rules: Vec<PolicyRule>,
    default_deny: bool,
}

impl PolicyEngine {
    pub fn new(default_deny: bool) -> Self {
        Self {
            rules: Vec::new(),
            default_deny,
        }
    }

    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
    }

    pub fn evaluate(&self, action: &str, subject: &str) -> PolicyDecision {
        for rule in &self.rules {
            if rule.action == action && rule.subject == subject {
                return rule.decision.clone();
            }
        }
        if self.default_deny {
            PolicyDecision::Deny
        } else {
            PolicyDecision::Allow
        }
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new(true)
    }
}
