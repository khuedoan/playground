----------------------- MODULE AgentOrbOwnership -----------------------
EXTENDS Naturals, FiniteSets

\* Finite safety model for personal agent-owned compute.
\* One agent can own many orbs. Each orb has exactly one owner.
\* Agents communicate through messages. Messages contain no orb authority.
\* Orbs use separate capabilities to connect to shared environments.

CONSTANTS Agents, Orbs, Resources, Commands, Messages, Environments, Effects,
          MaxEpoch, MaxBudget, MaxAttempts, SafeCommit

None == "none"

Idle == "idle"
Issued == "issued"
Committed == "committed"
Rejected == "rejected"
CommandStates == {Idle, Issued, Committed, Rejected}

MessageAbsent == "messageAbsent"
MessageSent == "messageSent"
MessageDelivered == "messageDelivered"
MessageStates == {MessageAbsent, MessageSent, MessageDelivered}

EffectIdle == "effectIdle"
EffectReserved == "effectReserved"
EffectSubmitted == "effectSubmitted"
EffectUnknown == "effectUnknown"
EffectConfirmed == "effectConfirmed"
EffectCancelled == "effectCancelled"
EffectStates == {EffectIdle, EffectReserved, EffectSubmitted,
                 EffectUnknown, EffectConfirmed, EffectCancelled}

\* The checked configurations use at most two resources.
Home(r) == IF r = "r1" THEN "o1" ELSE "o2"

VARIABLES
  orbOwner,
  running, orbGeneration, policyEpoch,
  sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
  resourceOwner, resourceFence, resourceVersion,
  commandStatus, commandAgent, commandOrb, commandResource,
  commandGeneration, commandOwnerEpoch, commandPolicy,
  commandFence, commandVersion, staleCommitAccepted,
  messageState, messageFrom, messageTo,
  environmentConnected, environmentEpoch,
  effectState, effectAgent, effectOrb, effectEnvironment,
  effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
  effectAttempts, effectOccurrences, retryGrants, retryUses, initialSubmitted,
  budgetAvailable, effectEscrow, effectSpent

vars == <<
  orbOwner,
  running, orbGeneration, policyEpoch,
  sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
  resourceOwner, resourceFence, resourceVersion,
  commandStatus, commandAgent, commandOrb, commandResource,
  commandGeneration, commandOwnerEpoch, commandPolicy,
  commandFence, commandVersion, staleCommitAccepted,
  messageState, messageFrom, messageTo,
  environmentConnected, environmentEpoch,
  effectState, effectAgent, effectOrb, effectEnvironment,
  effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
  effectAttempts, effectOccurrences, retryGrants, retryUses, initialSubmitted,
  budgetAvailable, effectEscrow, effectSpent
>>

OwnerSessionCurrent(o) ==
  /\ o \in Orbs
  /\ running[o]
  /\ sessionActive[o]
  /\ sessionStamp[o] = ownerEpoch[o]
  /\ sessionGeneration[o] = orbGeneration[o]
  /\ sessionPolicy[o] = policyEpoch[o]

EnvironmentCurrent(o, env) ==
  /\ o \in Orbs
  /\ env \in Environments
  /\ environmentConnected[o][env]

Init ==
  /\ orbOwner \in [Orbs -> Agents]
  /\ running = [o \in Orbs |-> TRUE]
  /\ orbGeneration = [o \in Orbs |-> 0]
  /\ policyEpoch = [o \in Orbs |-> 0]
  /\ sessionActive = [o \in Orbs |-> FALSE]
  /\ ownerEpoch = [o \in Orbs |-> 0]
  /\ sessionStamp = [o \in Orbs |-> 0]
  /\ sessionGeneration = [o \in Orbs |-> 0]
  /\ sessionPolicy = [o \in Orbs |-> 0]
  /\ resourceOwner = [r \in Resources |-> None]
  /\ resourceFence = [r \in Resources |-> 0]
  /\ resourceVersion = [r \in Resources |-> 0]
  /\ commandStatus = [c \in Commands |-> Idle]
  /\ commandAgent = [c \in Commands |-> None]
  /\ commandOrb = [c \in Commands |-> None]
  /\ commandResource = [c \in Commands |-> None]
  /\ commandGeneration = [c \in Commands |-> 0]
  /\ commandOwnerEpoch = [c \in Commands |-> 0]
  /\ commandPolicy = [c \in Commands |-> 0]
  /\ commandFence = [c \in Commands |-> 0]
  /\ commandVersion = [c \in Commands |-> 0]
  /\ staleCommitAccepted = FALSE
  /\ messageState = [m \in Messages |-> MessageAbsent]
  /\ messageFrom = [m \in Messages |-> None]
  /\ messageTo = [m \in Messages |-> None]
  /\ environmentConnected = [o \in Orbs |-> [env \in Environments |-> FALSE]]
  /\ environmentEpoch = [o \in Orbs |-> [env \in Environments |-> 0]]
  /\ effectState = [e \in Effects |-> EffectIdle]
  /\ effectAgent = [e \in Effects |-> None]
  /\ effectOrb = [e \in Effects |-> None]
  /\ effectEnvironment = [e \in Effects |-> None]
  /\ effectGeneration = [e \in Effects |-> 0]
  /\ effectOwnerEpoch = [e \in Effects |-> 0]
  /\ effectPolicy = [e \in Effects |-> 0]
  /\ effectEnvironmentEpoch = [e \in Effects |-> 0]
  /\ effectAttempts = [e \in Effects |-> 0]
  /\ effectOccurrences = [e \in Effects |-> 0]
  /\ retryGrants = [e \in Effects |-> 0]
  /\ retryUses = [e \in Effects |-> 0]
  /\ initialSubmitted = [e \in Effects |-> FALSE]
  /\ budgetAvailable = [a \in Agents |-> MaxBudget]
  /\ effectEscrow = [e \in Effects |-> 0]
  /\ effectSpent = [e \in Effects |-> 0]

\* Trusted supervisor actions. They cannot change orbOwner.
StartOwnerSession(o) ==
  /\ o \in Orbs
  /\ running[o]
  /\ ~OwnerSessionCurrent(o)
  /\ sessionActive' = [sessionActive EXCEPT ![o] = TRUE]
  /\ sessionStamp' = [sessionStamp EXCEPT ![o] = ownerEpoch[o]]
  /\ sessionGeneration' = [sessionGeneration EXCEPT ![o] = orbGeneration[o]]
  /\ sessionPolicy' = [sessionPolicy EXCEPT ![o] = policyEpoch[o]]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch, ownerEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

RevokeOwnerSession(o) ==
  /\ OwnerSessionCurrent(o)
  /\ ownerEpoch[o] < MaxEpoch
  /\ \A r \in Resources : Home(r) = o => resourceFence[r] < MaxEpoch
  /\ sessionActive' = [sessionActive EXCEPT ![o] = FALSE]
  /\ ownerEpoch' = [ownerEpoch EXCEPT ![o] = @ + 1]
  /\ resourceOwner' = [r \in Resources |->
       IF Home(r) = o THEN None ELSE resourceOwner[r]]
  /\ resourceFence' = [r \in Resources |->
       IF Home(r) = o THEN resourceFence[r] + 1 ELSE resourceFence[r]]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionStamp, sessionGeneration, sessionPolicy, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

FailGeneration(o) ==
  /\ o \in Orbs
  /\ running[o]
  /\ ownerEpoch[o] < MaxEpoch
  /\ \A r \in Resources : Home(r) = o => resourceFence[r] < MaxEpoch
  /\ running' = [running EXCEPT ![o] = FALSE]
  /\ sessionActive' = [sessionActive EXCEPT ![o] = FALSE]
  /\ ownerEpoch' = [ownerEpoch EXCEPT ![o] = @ + 1]
  /\ resourceOwner' = [r \in Resources |->
       IF Home(r) = o THEN None ELSE resourceOwner[r]]
  /\ resourceFence' = [r \in Resources |->
       IF Home(r) = o THEN resourceFence[r] + 1 ELSE resourceFence[r]]
  /\ UNCHANGED <<orbOwner, orbGeneration, policyEpoch,
       sessionStamp, sessionGeneration, sessionPolicy, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

RecoverGeneration(o) ==
  /\ o \in Orbs
  /\ ~running[o]
  /\ orbGeneration[o] < MaxEpoch
  /\ running' = [running EXCEPT ![o] = TRUE]
  /\ orbGeneration' = [orbGeneration EXCEPT ![o] = @ + 1]
  /\ UNCHANGED <<orbOwner, policyEpoch, sessionActive, ownerEpoch,
       sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

AdvancePolicy(o) ==
  /\ o \in Orbs
  /\ policyEpoch[o] < MaxEpoch
  /\ \A r \in Resources : Home(r) = o => resourceFence[r] < MaxEpoch
  /\ policyEpoch' = [policyEpoch EXCEPT ![o] = @ + 1]
  /\ resourceOwner' = [r \in Resources |->
       IF Home(r) = o THEN None ELSE resourceOwner[r]]
  /\ resourceFence' = [r \in Resources |->
       IF Home(r) = o THEN resourceFence[r] + 1 ELSE resourceFence[r]]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, sessionActive, ownerEpoch,
       sessionStamp, sessionGeneration, sessionPolicy, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

SendMessage(m, fromAgent, toAgent) ==
  /\ m \in Messages
  /\ fromAgent \in Agents
  /\ toAgent \in Agents
  /\ messageState[m] = MessageAbsent
  /\ messageState' = [messageState EXCEPT ![m] = MessageSent]
  /\ messageFrom' = [messageFrom EXCEPT ![m] = fromAgent]
  /\ messageTo' = [messageTo EXCEPT ![m] = toAgent]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

DeliverMessage(m) ==
  /\ m \in Messages
  /\ messageState[m] = MessageSent
  /\ messageState' = [messageState EXCEPT ![m] = MessageDelivered]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageFrom, messageTo, environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

ConnectEnvironment(o, env) ==
  /\ o \in Orbs
  /\ env \in Environments
  /\ ~environmentConnected[o][env]
  /\ environmentConnected' = [environmentConnected EXCEPT ![o][env] = TRUE]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

DisconnectEnvironment(o, env) ==
  /\ EnvironmentCurrent(o, env)
  /\ environmentEpoch[o][env] < MaxEpoch
  /\ environmentConnected' = [environmentConnected EXCEPT ![o][env] = FALSE]
  /\ environmentEpoch' = [environmentEpoch EXCEPT ![o][env] = @ + 1]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

Acquire(a, o, r) ==
  /\ OwnerSessionCurrent(o)
  /\ a = orbOwner[o]
  /\ r \in Resources
  /\ Home(r) = o
  /\ resourceOwner[r] = None
  /\ resourceOwner' = [resourceOwner EXCEPT ![r] = a]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

Issue(c, a, o, r) ==
  /\ c \in Commands
  /\ commandStatus[c] = Idle
  /\ OwnerSessionCurrent(o)
  /\ a = orbOwner[o]
  /\ r \in Resources
  /\ Home(r) = o
  /\ resourceOwner[r] = a
  /\ commandStatus' = [commandStatus EXCEPT ![c] = Issued]
  /\ commandAgent' = [commandAgent EXCEPT ![c] = a]
  /\ commandOrb' = [commandOrb EXCEPT ![c] = o]
  /\ commandResource' = [commandResource EXCEPT ![c] = r]
  /\ commandGeneration' = [commandGeneration EXCEPT ![c] = orbGeneration[o]]
  /\ commandOwnerEpoch' = [commandOwnerEpoch EXCEPT ![c] = ownerEpoch[o]]
  /\ commandPolicy' = [commandPolicy EXCEPT ![c] = policyEpoch[o]]
  /\ commandFence' = [commandFence EXCEPT ![c] = resourceFence[r]]
  /\ commandVersion' = [commandVersion EXCEPT ![c] = resourceVersion[r]]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

CurrentAtCommit(c) ==
  LET a == commandAgent[c]
      o == commandOrb[c]
      r == commandResource[c]
  IN /\ a \in Agents
     /\ o \in Orbs
     /\ r \in Resources
     /\ a = orbOwner[o]
     /\ Home(r) = o
     /\ OwnerSessionCurrent(o)
     /\ resourceOwner[r] = a
     /\ commandGeneration[c] = orbGeneration[o]
     /\ commandOwnerEpoch[c] = ownerEpoch[o]
     /\ commandPolicy[c] = policyEpoch[o]
     /\ commandFence[c] = resourceFence[r]
     /\ commandVersion[c] = resourceVersion[r]

Commit(c) ==
  /\ c \in Commands
  /\ commandStatus[c] = Issued
  /\ IF SafeCommit THEN CurrentAtCommit(c) ELSE TRUE
  /\ commandStatus' = [commandStatus EXCEPT ![c] = Committed]
  /\ resourceVersion' =
       IF CurrentAtCommit(c)
       THEN [resourceVersion EXCEPT ![commandResource[c]] = @ + 1]
       ELSE resourceVersion
  /\ staleCommitAccepted' = (staleCommitAccepted \lor ~CurrentAtCommit(c))
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence,
       commandAgent, commandOrb, commandResource, commandGeneration,
       commandOwnerEpoch, commandPolicy, commandFence, commandVersion,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

RejectStale(c) ==
  /\ c \in Commands
  /\ commandStatus[c] = Issued
  /\ ~CurrentAtCommit(c)
  /\ commandStatus' = [commandStatus EXCEPT ![c] = Rejected]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandAgent, commandOrb, commandResource, commandGeneration,
       commandOwnerEpoch, commandPolicy, commandFence, commandVersion,
       staleCommitAccepted, messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

ReserveEffect(e, a, o, env) ==
  /\ e \in Effects
  /\ effectState[e] = EffectIdle
  /\ OwnerSessionCurrent(o)
  /\ a = orbOwner[o]
  /\ EnvironmentCurrent(o, env)
  /\ budgetAvailable[a] > 0
  /\ effectState' = [effectState EXCEPT ![e] = EffectReserved]
  /\ effectAgent' = [effectAgent EXCEPT ![e] = a]
  /\ effectOrb' = [effectOrb EXCEPT ![e] = o]
  /\ effectEnvironment' = [effectEnvironment EXCEPT ![e] = env]
  /\ effectGeneration' = [effectGeneration EXCEPT ![e] = orbGeneration[o]]
  /\ effectOwnerEpoch' = [effectOwnerEpoch EXCEPT ![e] = ownerEpoch[o]]
  /\ effectPolicy' = [effectPolicy EXCEPT ![e] = policyEpoch[o]]
  /\ effectEnvironmentEpoch' = [effectEnvironmentEpoch EXCEPT
       ![e] = environmentEpoch[o][env]]
  /\ budgetAvailable' = [budgetAvailable EXCEPT ![a] = @ - 1]
  /\ effectEscrow' = [effectEscrow EXCEPT ![e] = 1]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, effectSpent>>

EffectSubmissionCurrent(e) ==
  LET a == effectAgent[e]
      o == effectOrb[e]
      env == effectEnvironment[e]
  IN /\ a \in Agents
     /\ o \in Orbs
     /\ env \in Environments
     /\ a = orbOwner[o]
     /\ OwnerSessionCurrent(o)
     /\ EnvironmentCurrent(o, env)
     /\ effectGeneration[e] = orbGeneration[o]
     /\ effectOwnerEpoch[e] = ownerEpoch[o]
     /\ effectPolicy[e] = policyEpoch[o]
     /\ effectEnvironmentEpoch[e] = environmentEpoch[o][env]

SubmitEffect(e) ==
  /\ e \in Effects
  /\ effectState[e] = EffectReserved
  /\ EffectSubmissionCurrent(e)
  /\ effectAttempts[e] < MaxAttempts
  /\ IF effectAttempts[e] = 0 THEN TRUE ELSE retryUses[e] < retryGrants[e]
  /\ effectState' = [effectState EXCEPT ![e] = EffectSubmitted]
  /\ effectAttempts' = [effectAttempts EXCEPT ![e] = @ + 1]
  /\ initialSubmitted' =
       IF effectAttempts[e] = 0
       THEN [initialSubmitted EXCEPT ![e] = TRUE]
       ELSE initialSubmitted
  /\ retryUses' =
       IF effectAttempts[e] > 0
       THEN [retryUses EXCEPT ![e] = @ + 1]
       ELSE retryUses
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectAgent, effectOrb, effectEnvironment, effectGeneration,
       effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectOccurrences, retryGrants, budgetAvailable, effectEscrow, effectSpent>>

BrokerOccurs(e) ==
  /\ e \in Effects
  /\ effectState[e] = EffectSubmitted
  /\ effectOccurrences[e] = 0
  /\ effectOccurrences' = [effectOccurrences EXCEPT ![e] = 1]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectState, effectAgent, effectOrb, effectEnvironment,
       effectGeneration, effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

LoseOutcome(e) ==
  /\ e \in Effects
  /\ effectState[e] = EffectSubmitted
  /\ effectState' = [effectState EXCEPT ![e] = EffectUnknown]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectAgent, effectOrb, effectEnvironment, effectGeneration,
       effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

ConfirmOccurred(e) ==
  /\ e \in Effects
  /\ effectState[e] \in {EffectSubmitted, EffectUnknown}
  /\ effectOccurrences[e] = 1
  /\ effectState' = [effectState EXCEPT ![e] = EffectConfirmed]
  /\ effectEscrow' = [effectEscrow EXCEPT ![e] = 0]
  /\ effectSpent' = [effectSpent EXCEPT ![e] = 1]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectAgent, effectOrb, effectEnvironment, effectGeneration,
       effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable>>

\* The supervisor rebinds only a reconciled, non-occurring effect.
AuthorizeRetry(e) ==
  /\ e \in Effects
  /\ effectState[e] = EffectUnknown
  /\ effectOccurrences[e] = 0
  /\ retryGrants[e] < MaxAttempts - 1
  /\ LET o == effectOrb[e]
         env == effectEnvironment[e]
     IN /\ o \in Orbs
        /\ env \in Environments
        /\ OwnerSessionCurrent(o)
        /\ EnvironmentCurrent(o, env)
        /\ effectAgent[e] = orbOwner[o]
        /\ effectState' = [effectState EXCEPT ![e] = EffectReserved]
        /\ effectGeneration' = [effectGeneration EXCEPT ![e] = orbGeneration[o]]
        /\ effectOwnerEpoch' = [effectOwnerEpoch EXCEPT ![e] = ownerEpoch[o]]
        /\ effectPolicy' = [effectPolicy EXCEPT ![e] = policyEpoch[o]]
        /\ effectEnvironmentEpoch' = [effectEnvironmentEpoch EXCEPT
             ![e] = environmentEpoch[o][env]]
  /\ retryGrants' = [retryGrants EXCEPT ![e] = @ + 1]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectAgent, effectOrb, effectEnvironment,
       effectAttempts, effectOccurrences, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

CancelUnsubmitted(e) ==
  /\ e \in Effects
  /\ effectState[e] = EffectReserved
  /\ effectAttempts[e] = 0
  /\ effectAgent[e] \in Agents
  /\ effectState' = [effectState EXCEPT ![e] = EffectCancelled]
  /\ budgetAvailable' = [budgetAvailable EXCEPT ![effectAgent[e]] = @ + 1]
  /\ effectEscrow' = [effectEscrow EXCEPT ![e] = 0]
  /\ UNCHANGED <<orbOwner, running, orbGeneration, policyEpoch,
       sessionActive, ownerEpoch, sessionStamp, sessionGeneration, sessionPolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandOwnerEpoch, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       messageState, messageFrom, messageTo,
       environmentConnected, environmentEpoch,
       effectAgent, effectOrb, effectEnvironment, effectGeneration,
       effectOwnerEpoch, effectPolicy, effectEnvironmentEpoch,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, effectSpent>>

Next ==
  \/ \E o \in Orbs : StartOwnerSession(o)
  \/ \E o \in Orbs : RevokeOwnerSession(o)
  \/ \E o \in Orbs : FailGeneration(o)
  \/ \E o \in Orbs : RecoverGeneration(o)
  \/ \E o \in Orbs : AdvancePolicy(o)
  \/ \E m \in Messages, a \in Agents, b \in Agents : SendMessage(m, a, b)
  \/ \E m \in Messages : DeliverMessage(m)
  \/ \E o \in Orbs, env \in Environments : ConnectEnvironment(o, env)
  \/ \E o \in Orbs, env \in Environments : DisconnectEnvironment(o, env)
  \/ \E a \in Agents, o \in Orbs, r \in Resources : Acquire(a, o, r)
  \/ \E c \in Commands, a \in Agents, o \in Orbs, r \in Resources :
       Issue(c, a, o, r)
  \/ \E c \in Commands : Commit(c)
  \/ \E c \in Commands : RejectStale(c)
  \/ \E e \in Effects, a \in Agents, o \in Orbs, env \in Environments :
       ReserveEffect(e, a, o, env)
  \/ \E e \in Effects : SubmitEffect(e)
  \/ \E e \in Effects : BrokerOccurs(e)
  \/ \E e \in Effects : LoseOutcome(e)
  \/ \E e \in Effects : ConfirmOccurred(e)
  \/ \E e \in Effects : AuthorizeRetry(e)
  \/ \E e \in Effects : CancelUnsubmitted(e)

Spec == Init /\ [][Next]_vars

TypeOK ==
  /\ orbOwner \in [Orbs -> Agents]
  /\ running \in [Orbs -> BOOLEAN]
  /\ orbGeneration \in [Orbs -> 0..MaxEpoch]
  /\ policyEpoch \in [Orbs -> 0..MaxEpoch]
  /\ sessionActive \in [Orbs -> BOOLEAN]
  /\ ownerEpoch \in [Orbs -> 0..MaxEpoch]
  /\ sessionStamp \in [Orbs -> 0..MaxEpoch]
  /\ sessionGeneration \in [Orbs -> 0..MaxEpoch]
  /\ sessionPolicy \in [Orbs -> 0..MaxEpoch]
  /\ resourceOwner \in [Resources -> Agents \cup {None}]
  /\ resourceFence \in [Resources -> Nat]
  /\ resourceVersion \in [Resources -> Nat]
  /\ commandStatus \in [Commands -> CommandStates]
  /\ commandAgent \in [Commands -> Agents \cup {None}]
  /\ commandOrb \in [Commands -> Orbs \cup {None}]
  /\ commandResource \in [Commands -> Resources \cup {None}]
  /\ commandGeneration \in [Commands -> 0..MaxEpoch]
  /\ commandOwnerEpoch \in [Commands -> 0..MaxEpoch]
  /\ commandPolicy \in [Commands -> 0..MaxEpoch]
  /\ commandFence \in [Commands -> Nat]
  /\ commandVersion \in [Commands -> Nat]
  /\ staleCommitAccepted \in BOOLEAN
  /\ messageState \in [Messages -> MessageStates]
  /\ messageFrom \in [Messages -> Agents \cup {None}]
  /\ messageTo \in [Messages -> Agents \cup {None}]
  /\ environmentConnected \in [Orbs -> [Environments -> BOOLEAN]]
  /\ environmentEpoch \in [Orbs -> [Environments -> 0..MaxEpoch]]
  /\ effectState \in [Effects -> EffectStates]
  /\ effectAgent \in [Effects -> Agents \cup {None}]
  /\ effectOrb \in [Effects -> Orbs \cup {None}]
  /\ effectEnvironment \in [Effects -> Environments \cup {None}]
  /\ effectGeneration \in [Effects -> 0..MaxEpoch]
  /\ effectOwnerEpoch \in [Effects -> 0..MaxEpoch]
  /\ effectPolicy \in [Effects -> 0..MaxEpoch]
  /\ effectEnvironmentEpoch \in [Effects -> 0..MaxEpoch]
  /\ effectAttempts \in [Effects -> 0..MaxAttempts]
  /\ effectOccurrences \in [Effects -> 0..1]
  /\ retryGrants \in [Effects -> 0..MaxAttempts]
  /\ retryUses \in [Effects -> 0..MaxAttempts]
  /\ initialSubmitted \in [Effects -> BOOLEAN]
  /\ budgetAvailable \in [Agents -> 0..MaxBudget]
  /\ effectEscrow \in [Effects -> 0..1]
  /\ effectSpent \in [Effects -> 0..1]

OneOwnerPerOrb == orbOwner \in [Orbs -> Agents]

OwnerControlsResources ==
  \A r \in Resources :
    resourceOwner[r] = None \/
      LET o == Home(r)
      IN /\ o \in Orbs
         /\ resourceOwner[r] = orbOwner[o]
         /\ OwnerSessionCurrent(o)

NoForeignCommand ==
  \A c \in Commands :
    commandStatus[c] = Idle \/
      LET o == commandOrb[c]
      IN /\ o \in Orbs
         /\ commandAgent[c] = orbOwner[o]

NoStaleCommit == ~staleCommitAccepted

MessageWellFormed ==
  \A m \in Messages :
    /\ (messageState[m] = MessageAbsent =>
          messageFrom[m] = None /\ messageTo[m] = None)
    /\ (messageState[m] # MessageAbsent =>
          messageFrom[m] \in Agents /\ messageTo[m] \in Agents)

NoForeignEffect ==
  \A e \in Effects :
    effectState[e] = EffectIdle \/
      LET o == effectOrb[e]
      IN /\ o \in Orbs
         /\ effectAgent[e] = orbOwner[o]
         /\ effectEnvironment[e] \in Environments

BudgetConserved ==
  \A a \in Agents :
    budgetAvailable[a]
    + Cardinality({e \in Effects : effectAgent[e] = a /\ effectEscrow[e] = 1})
    + Cardinality({e \in Effects : effectAgent[e] = a /\ effectSpent[e] = 1})
    = MaxBudget

TerminalEscrowSettled ==
  \A e \in Effects :
    /\ effectEscrow[e] + effectSpent[e] <= 1
    /\ (effectState[e] = EffectConfirmed =>
          effectEscrow[e] = 0 /\ effectSpent[e] = 1)
    /\ (effectState[e] = EffectCancelled =>
          effectEscrow[e] = 0 /\ effectSpent[e] = 0)

RetryOnlyAfterReconciliation ==
  \A e \in Effects :
    /\ retryUses[e] <= retryGrants[e]
    /\ effectAttempts[e] =
         (IF initialSubmitted[e] THEN 1 ELSE 0) + retryUses[e]
    /\ effectOccurrences[e] <= 1

=============================================================================
