# Valorant-rs

A crate which offers unofficial valorant api client written in Rust.
use this when, for instance, u want to crate a valorant discord bot in Rust.

## am i getting banned?
[the original api's document]() mentions that:

> These endpoints are not officially supported. However, as long as you use common sense and don't do anything a Riot employee would frown at, you won't get banned.

I've been using this too while developing but haven't get banned so most likely you are safe but use at your own risk.

## endpoints coverage

Don't know if i cover all those tho.

XMPP
- [ ] TCP XMPP Connection

PVP ENDPOINTS
- [ ] GET Fetch Content
- [ ] GET Account XP
- [ ] GET Player Loadout
- [ ] PUT Set Player Loadout
- [ ] GET Player MMR
- [ ] GET Match History
- [ ] GET Match Details
- [ ] GET Competitive Updates
- [ ] GET Leaderboard
- [ ] GET Penalties
- [ ] GET Config
- [ ] PUT Name Service

PARTY ENDPOINTS
- [ ] GET Party
- [ ] GET Party Player
- [ ] DELETE Party Remove Player
- [ ] POST Party Set Member Ready
- [ ] POST Refresh Competitive Tier
- [ ] POST Refresh Player Identity
- [ ] POST Refresh Pings
- [ ] POST Change Queue
- [ ] POST Start Custom Game
- [ ] POST Enter Matchmaking Queue
- [ ] POST Leave Matchmaking Queue
- [ ] POST Set Party Accessibility
- [ ] POST Set Custom Game Settings
- [ ] POST Party Invite
- [ ] POST Party Request
- [ ] POST Party Decline
- [ ] GET Custom Game Configs
- [ ] GET Party Chat Token
- [ ] GET Party Voice Token
- [ ] DELETE Party Disable Code
- [ ] POST Party Generate Code
- [ ] POST Party Join By Code

STORE ENDPOINTS
- [x] GET Prices
- [x] GET Storefront
- [x] GET Wallet
- [x] GET Owned Items

PRE-GAME ENDPOINTS
- [ ] GET Pre-Game Player
- [ ] GET Pre-Game Match
- [ ] GET Pre-Game Loadouts
- [ ] POST Select Character
- [ ] POST Lock Character
- [ ] POST Pre-Game Quit

CURRENT GAME ENDPOINTS
- [x] GET Current Game Player
- [x] GET Current Game Match
- [x] GET Current Game Loadouts
- [x] POST Current Game Quit

CONTRACT ENDPOINTS
- [ ] GET Item Upgrades
- [ ] GET Contracts
- [ ] POST Activate Contract

LOCAL ENDPOINTS
- [ ] GET Local Help
- [ ] GET Sessions
- [ ] GET RSO User Info
- [ ] GET Client Region
- [ ] GET Account Alias
- [x] GET Entitlements Token
- [ ] GET Chat Session
- [ ] GET Friends
- [ ] POST Send Friend Request
- [ ] DELETE Remove Friend Request
- [ ] GET Presence
- [ ] GET Friend Requests
- [ ] GET Local Swagger Docs
- [ ] WSS Local WebSocket

LOCAL ENDPOINTS - CHAT
- [ ] GET Party Chat Info
- [ ] GET Pre-Game Chat Info
- [ ] GET Current Game Chat Info
- [ ] GET All Chat Info
- [ ] GET Chat Participants
- [ ] POST Send Chat
- [ ] GET Chat History

AUTHENTICATION ENDPOINTS
- [ ] POST Auth Cookies
- [ ] PUT Auth Request
- [ ] PUT Multi-Factor Authentication
- [ ] GET Cookie Reauth
- [ ] POST Entitlement
- [ ] GET Player Info
- [ ] PUT Riot Geo
- [ ] GET PAS Token
- [ ] GET Riot Client Config