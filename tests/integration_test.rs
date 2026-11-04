use sigmaos_policy::{Config, PolicyComponent, PolicyDecision, PolicyEngine, PolicyRule, Component};

#[test]
fn test_config_default_aaa() {
    let config = Config::default();
    assert!(config.enabled);
    assert!(config.default_deny);
}

#[test]
fn test_policy_engine_default_deny_aaa() {
    // Arrange
    let engine = PolicyEngine::new(true);
    // Act
    let decision = engine.evaluate("org.sigma.admin", "user:alice");
    // Assert
    assert_eq!(decision, PolicyDecision::Deny);
}

#[test]
fn test_policy_engine_allow_rule_aaa() {
    // Arrange
    let mut engine = PolicyEngine::new(true);
    engine.add_rule(PolicyRule::new("org.sigma.admin", "user:alice", PolicyDecision::Allow));
    // Act
    let decision = engine.evaluate("org.sigma.admin", "user:alice");
    // Assert
    assert_eq!(decision, PolicyDecision::Allow);
}

#[test]
fn test_policy_engine_rule_count_aaa() {
    // Arrange
    let mut engine = PolicyEngine::default();
    // Act
    engine.add_rule(PolicyRule::new("action1", "subject1", PolicyDecision::Allow));
    engine.add_rule(PolicyRule::new("action2", "subject2", PolicyDecision::Deny));
    // Assert
    assert_eq!(engine.rule_count(), 2);
}

#[test]
fn test_policy_engine_challenge_aaa() {
    // Arrange
    let mut engine = PolicyEngine::new(true);
    engine.add_rule(PolicyRule::new("org.sigma.settings", "user:regular", PolicyDecision::Challenge));
    // Act
    let decision = engine.evaluate("org.sigma.settings", "user:regular");
    // Assert
    assert_eq!(decision, PolicyDecision::Challenge);
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut component = PolicyComponent::new(Config::default());
    assert!(component.initialize().is_ok());
    assert!(component.health_check().is_ok());
    assert!(component.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let component = PolicyComponent::new(Config::default());
    assert_eq!(component.name(), "sigmaos-policy");
}
