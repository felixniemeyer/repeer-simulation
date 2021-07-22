use std::collections::HashMap; 
use rand::Rng;

type AgentId = usize; 
type Reputation = i64; 

type RequestResponse = bool; 
const ACCEPT: RequestResponse = true; 
const REJECT: RequestResponse = false; 

#[derive(Clone, Copy)]
struct Game {
    requester: AgentId, 
    responder: AgentId, 
}

trait ResponseStrategy {
    fn review_request(&mut self, requester_id: AgentId) -> RequestResponse; 
    fn get_type(&self) -> &str;
}

struct ReputationTracker {
    reputations: HashMap<AgentId, Reputation>, 
    lower_threshold: Reputation
}
impl ReputationTracker {
    const ACCEPT_COSTS: Reputation = 1; 
    fn blank() -> ReputationTracker {
        ReputationTracker {
            reputations: HashMap::<AgentId, Reputation>::new(), 
            lower_threshold: 0
        }
    }
}

impl ResponseStrategy for ReputationTracker {
    fn review_request(&mut self, requester_id: AgentId) -> RequestResponse {
        match self.reputations.get_mut(&requester_id) {
            Some(r) => {
                if *r >= self.lower_threshold { 
                    *r -= Self::ACCEPT_COSTS; 
                    ACCEPT
                } else { 
                    REJECT 
                }
            }, 
            None => {
                self.reputations.insert(requester_id, -Self::ACCEPT_COSTS);
                ACCEPT
            }
        }
    }
    fn get_type(&self) -> &str { "reputation tracker" }
}
 
struct AlwaysAccept {}
impl ResponseStrategy for AlwaysAccept{
    fn review_request(&mut self, _requester_id: AgentId) -> RequestResponse { ACCEPT }
    fn get_type(&self) -> &str { "accepter" }
}

struct AlwaysReject {}
impl ResponseStrategy for AlwaysReject {
    fn review_request(&mut self, _requester_id: AgentId) -> RequestResponse { REJECT }
    fn get_type(&self) -> &str { "rejecter" }
}

struct Alternate {
    last_response: RequestResponse
}
impl ResponseStrategy for Alternate{
    fn review_request(&mut self, _requester_id: AgentId) -> RequestResponse { 
        self.last_response = !self.last_response;
        self.last_response
    }
    fn get_type(&self) -> &str { "rejecter" }
}

struct Agent {
    response_strategy: Box<dyn ResponseStrategy>, 
    // exploit_strategy: Box<dyn ExploitStrategy>
    lifes: i64, 
}

fn main() {
    const NUM_AGENT_TYPES: usize = 4; 
    const NUM_AGENTS: usize = 100 * NUM_AGENT_TYPES; 
    let mut agents: Vec<Agent> = vec![];
    for _ in 0..NUM_AGENTS / NUM_AGENT_TYPES {
        let strategies : [Box<dyn ResponseStrategy>; NUM_AGENT_TYPES] = [
            Box::new(ReputationTracker::blank()), 
            Box::new(AlwaysAccept {}),
            Box::new(AlwaysReject {}),
            Box::new(Alternate {last_response: REJECT})
        ];
        for strategy in strategies {
            agents.push(Agent {
                response_strategy: strategy, 
                lifes: 100
            })
        }
    }
    
    println!("agent len(): {}", agents.len());
    
    let mut rng = rand::thread_rng();

    for round in 0..1000 {
        let mut games: [Game; NUM_AGENTS * 10] = [Game{
            requester: 0, 
            responder: 0 
        }; NUM_AGENTS * 10];
        for (agent_id, _) in agents.iter_mut().enumerate() {
             for game_local_id in 0..10 {
                 let game_id = game_local_id + agent_id * 10;
                 games[game_id].requester = agent_id; 
                 let mut responder = rng.gen::<usize>() % (NUM_AGENTS - 1);
                 if responder == agent_id {
                     responder += 1;
                 }
                 games[game_id].responder = responder;
             }
        }
        for game in games {
            let mut conduct_game = false;
            {
                let responder = agents.get_mut(game.responder).unwrap(); 
                if responder.response_strategy.review_request(game.requester) {
                    conduct_game = true; 
                    responder.lifes -= 1;
                }
            }
            let requester = agents.get_mut(game.requester).unwrap();
            if conduct_game {
                requester.lifes += 5;
            }
        }
        
        println!("agent 0 has {} lifes after round {}", agents[0].lifes, round);
    }
}
