use crate::profiles::NetworkProfile;
use soroban_sdk::Env;

pub struct TestContextBuilder {
    num_users: usize,
    mock_auths: bool,
    network_profile: NetworkProfile,
}

impl Default for TestContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TestContextBuilder {
    pub fn new() -> Self {
        Self {
            num_users: 0,
            mock_auths: true,
            network_profile: NetworkProfile::Local,
        }
    }

    pub fn with_users(mut self, count: usize) -> Self {
        self.num_users = count;
        self
    }

    /// Select the network environment to emulate.
    ///
    /// The default is [`NetworkProfile::Local`], which keeps the test host
    /// unmetered for fast local iteration.
    pub fn with_network_profile(mut self, profile: NetworkProfile) -> Self {
        self.network_profile = profile;
        self
    }

    pub fn without_mock_auths(mut self) -> Self {
        self.mock_auths = false;
        self
    }

    pub fn build(self) -> super::TestContext {
        let mut ctx = super::TestContext::new();
        if !self.mock_auths {
            ctx.env = Env::default();
        }
        self.network_profile.apply(&ctx.env);
        for _ in 0..self.num_users {
            ctx.add_user();
        }
        ctx
    }
}
