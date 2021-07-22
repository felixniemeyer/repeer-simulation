## repeer simulation

The idea is to explore a world without money and just agent centric reputation tracking. 
Every agent believes noone but himself about who he can trust. 
When transactions have been positive, the agent rises the reputation of the transaction partner. 
This way, agents avoid transactions with agents that frequently defect. 

This is a simulation allowing various strategies for deciding whether to trust someone to be implemented. 
Two player games are randomly offered to agents. Agents can then decide, whether they will participate in the game. 
At the moment, The games have a payout to only one player and even costs to the other player. The payout is substantially higher than the costs. 
If the opponent would be anonymous, there would be no reason for the player that looses. If there would be some mechanism to ensure the other player would act the same way next time, it would be a great win-win. But there is no way to make contracts and the only thing people can try to figure out is: "does this player play a strategy such that I might profit in our next encounter"...

Possibilities what to explore are wide open. 
E.g. in reality, agents might want to estimate, how likely it is to encounter an opponent of such a 2 player game again. 

It's anarchy. We're simulating anarchy. 
And that's because, this question is a big one in decentral anonymous networks. E.g. "can I trust this store on the dark web?". But it's nice: when everyone has a private key as identification, everyone else can track their reputations from his or her perspective. 

Something that I will definitely want to explore is: asking peers you trust for references when interacting with an agent. Really, it's close to real life: referring employees, the friend of your friend is your friend. 

There could emerge "institutions" that people can trust (like it already is in real life: people trust amazon for managing reviews well, eliminating fakes etc.).


### p2p realization
Eventually, this may develop into a decentral app powered by libp2p, that allows apps to implemented on top. There could be multiple dimensions of reputation. Apps could allow users to conveniently give feedback after transactions. Because that's the whole point: no system can figure out, whether sb. is reputable or not. At the leaves it always relies on personal opinions whether e.g. a promised value has been delivered. 
