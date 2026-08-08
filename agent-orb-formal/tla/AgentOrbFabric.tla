-------------------------- MODULE AgentOrbFabric --------------------------
EXTENDS Naturals, FiniteSets

\* Finite abstraction of the Agent-Orb Fabric control protocol.
\* This model covers admission/capabilities, revocation, orb generation
\* recovery, resource fencing/version-CAS, broker effects, retries, and
\* budget escrow. It intentionally does not model checkpoints or causal
\* consistency; no claim about either follows from this module.

CONSTANTS Agents, Orbs, Resources, Commands, Effects,
          MaxEpoch, MaxBudget, MaxAttempts

None       == "none"
Observer   == "observer"
Worker     == "worker"
Admin      == "admin"
Roles      == {Observer, Worker, Admin}

Idle       == "idle"
Issued     == "issued"
Committed  == "committed"
Rejected   == "rejected"

EffectIdle       == "effectIdle"
EffectReserved   == "effectReserved"
EffectSubmitted  == "effectSubmitted"
EffectUnknown    == "effectUnknown"
EffectConfirmed  == "effectConfirmed"
EffectCancelled  == "effectCancelled"
EffectStates     == {EffectIdle, EffectReserved, EffectSubmitted,
                    EffectUnknown, EffectConfirmed, EffectCancelled}

\* The small checked instance uses one resource. Keeping the association
\* explicit in the model makes an accepted command orb-bound.
Home(r) == IF r = "r1" THEN "o1" ELSE "o2"

VARIABLES
  running, orbGeneration, policyEpoch,
  active, role, presenceEpoch, presenceStamp,
  presenceGeneration, presencePolicy,
  resourceOwner, resourceFence, resourceVersion,
  commandStatus, commandAgent, commandOrb, commandResource,
  commandGeneration, commandPresence, commandPolicy,
  commandFence, commandVersion, staleCommitAccepted,
  effectState, effectAgent, effectOrb, effectGeneration,
  effectPresence, effectPolicy, effectAttempts,
  effectOccurrences, retryGrants, retryUses, initialSubmitted,
  budgetAvailable, effectEscrow, effectSpent

vars == <<
  running, orbGeneration, policyEpoch,
  active, role, presenceEpoch, presenceStamp,
  presenceGeneration, presencePolicy,
  resourceOwner, resourceFence, resourceVersion,
  commandStatus, commandAgent, commandOrb, commandResource,
  commandGeneration, commandPresence, commandPolicy,
  commandFence, commandVersion, staleCommitAccepted,
  effectState, effectAgent, effectOrb, effectGeneration,
  effectPresence, effectPolicy, effectAttempts,
  effectOccurrences, retryGrants, retryUses, initialSubmitted,
  budgetAvailable, effectEscrow, effectSpent
>>

RoleCanMutate(r) == r \in {Worker, Admin}
RoleCanEffect(r) == r \in {Worker, Admin}

PresenceCurrent(a, o) ==
  /\ running[o]
  /\ active[a][o]
  /\ presenceStamp[a][o] = presenceEpoch[a][o]
  /\ presenceGeneration[a][o] = orbGeneration[o]
  /\ presencePolicy[a][o] = policyEpoch[o]

Init ==
  /\ running = [o \in Orbs |-> TRUE]
  /\ orbGeneration = [o \in Orbs |-> 0]
  /\ policyEpoch = [o \in Orbs |-> 0]
  /\ active = [a \in Agents |-> [o \in Orbs |-> FALSE]]
  /\ role = [a \in Agents |-> [o \in Orbs |-> None]]
  /\ presenceEpoch = [a \in Agents |-> [o \in Orbs |-> 0]]
  /\ presenceStamp = [a \in Agents |-> [o \in Orbs |-> 0]]
  /\ presenceGeneration = [a \in Agents |-> [o \in Orbs |-> 0]]
  /\ presencePolicy = [a \in Agents |-> [o \in Orbs |-> 0]]
  /\ resourceOwner = [r \in Resources |-> None]
  /\ resourceFence = [r \in Resources |-> 0]
  /\ resourceVersion = [r \in Resources |-> 0]
  /\ commandStatus = [c \in Commands |-> Idle]
  /\ commandAgent = [c \in Commands |-> None]
  /\ commandOrb = [c \in Commands |-> None]
  /\ commandResource = [c \in Commands |-> None]
  /\ commandGeneration = [c \in Commands |-> 0]
  /\ commandPresence = [c \in Commands |-> 0]
  /\ commandPolicy = [c \in Commands |-> 0]
  /\ commandFence = [c \in Commands |-> 0]
  /\ commandVersion = [c \in Commands |-> 0]
  /\ staleCommitAccepted = FALSE
  /\ effectState = [e \in Effects |-> EffectIdle]
  /\ effectAgent = [e \in Effects |-> None]
  /\ effectOrb = [e \in Effects |-> None]
  /\ effectGeneration = [e \in Effects |-> 0]
  /\ effectPresence = [e \in Effects |-> 0]
  /\ effectPolicy = [e \in Effects |-> 0]
  /\ effectAttempts = [e \in Effects |-> 0]
  /\ effectOccurrences = [e \in Effects |-> 0]
  /\ retryGrants = [e \in Effects |-> 0]
  /\ retryUses = [e \in Effects |-> 0]
  /\ initialSubmitted = [e \in Effects |-> FALSE]
  /\ budgetAvailable = [a \in Agents |-> MaxBudget]
  /\ effectEscrow = [e \in Effects |-> 0]
  /\ effectSpent = [e \in Effects |-> 0]

\* Admission is a trusted supervisor input. The model proves consequences of
\* a linearized admission; it does not prove the supervisor's own identity.
SupervisorAdmit(a, o, newRole) ==
  /\ a \in Agents
  /\ o \in Orbs
  /\ newRole \in Roles
  /\ running[o]
  /\ ~PresenceCurrent(a, o)
  /\ active' = [active EXCEPT ![a][o] = TRUE]
  /\ role' = [role EXCEPT ![a][o] = newRole]
  /\ presenceStamp' = [presenceStamp EXCEPT
       ![a][o] = presenceEpoch[a][o]]
  /\ presenceGeneration' = [presenceGeneration EXCEPT
       ![a][o] = orbGeneration[o]]
  /\ presencePolicy' = [presencePolicy EXCEPT
       ![a][o] = policyEpoch[o]]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts, effectOccurrences,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

SupervisorRevoke(a, o) ==
  /\ PresenceCurrent(a, o)
  /\ presenceEpoch[a][o] < MaxEpoch
  /\ \A r \in Resources :
       resourceOwner[r] = a /\ Home(r) = o => resourceFence[r] < MaxEpoch
  /\ active' = [active EXCEPT ![a][o] = FALSE]
  /\ role' = [role EXCEPT ![a][o] = None]
  /\ presenceEpoch' = [presenceEpoch EXCEPT ![a][o] = @ + 1]
  /\ resourceOwner' = [r \in Resources |->
       IF resourceOwner[r] = a /\ Home(r) = o THEN None
       ELSE resourceOwner[r]]
  /\ resourceFence' = [r \in Resources |->
       IF resourceOwner[r] = a /\ Home(r) = o
       THEN resourceFence[r] + 1 ELSE resourceFence[r]]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch,
       presenceStamp, presenceGeneration, presencePolicy, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts, effectOccurrences,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

FailGeneration(o) ==
  /\ o \in Orbs
  /\ running[o]
  /\ \A a \in Agents : presenceEpoch[a][o] < MaxEpoch
  /\ \A r \in Resources : Home(r) = o => resourceFence[r] < MaxEpoch
  /\ running' = [running EXCEPT ![o] = FALSE]
  /\ active' = [a \in Agents |->
       [active[a] EXCEPT ![o] = FALSE]]
  /\ role' = [a \in Agents |-> [role[a] EXCEPT ![o] = None]]
  /\ presenceEpoch' = [a \in Agents |->
       [presenceEpoch[a] EXCEPT ![o] = @ + 1]]
  /\ resourceOwner' = [r \in Resources |->
       IF Home(r) = o THEN None ELSE resourceOwner[r]]
  /\ resourceFence' = [r \in Resources |->
       IF Home(r) = o THEN resourceFence[r] + 1 ELSE resourceFence[r]]
  /\ UNCHANGED <<orbGeneration, policyEpoch,
       presenceStamp, presenceGeneration, presencePolicy, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts, effectOccurrences,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

RecoverGeneration(o) ==
  /\ o \in Orbs
  /\ ~running[o]
  /\ orbGeneration[o] < MaxEpoch
  /\ running' = [running EXCEPT ![o] = TRUE]
  /\ orbGeneration' = [orbGeneration EXCEPT ![o] = @ + 1]
  /\ UNCHANGED <<policyEpoch, active, role, presenceEpoch,
       presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts, effectOccurrences,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

AdvancePolicy(o) ==
  /\ o \in Orbs
  /\ policyEpoch[o] < MaxEpoch
  /\ \A r \in Resources : Home(r) = o => resourceFence[r] < MaxEpoch
  /\ policyEpoch' = [policyEpoch EXCEPT ![o] = @ + 1]
  /\ resourceOwner' = [r \in Resources |->
       IF Home(r) = o THEN None ELSE resourceOwner[r]]
  /\ resourceFence' = [r \in Resources |->
       IF Home(r) = o THEN resourceFence[r] + 1 ELSE resourceFence[r]]
  /\ UNCHANGED <<running, orbGeneration, active, role, presenceEpoch,
       presenceStamp, presenceGeneration, presencePolicy, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts, effectOccurrences,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

Acquire(a, o, r) ==
  /\ PresenceCurrent(a, o)
  /\ RoleCanMutate(role[a][o])
  /\ r \in Resources
  /\ Home(r) = o
  /\ resourceOwner[r] = None
  /\ resourceOwner' = [resourceOwner EXCEPT ![r] = a]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts, effectOccurrences,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

Issue(c, a, o, r) ==
  /\ c \in Commands
  /\ commandStatus[c] = Idle
  /\ PresenceCurrent(a, o)
  /\ RoleCanMutate(role[a][o])
  /\ r \in Resources
  /\ Home(r) = o
  /\ resourceOwner[r] = a
  /\ commandStatus' = [commandStatus EXCEPT ![c] = Issued]
  /\ commandAgent' = [commandAgent EXCEPT ![c] = a]
  /\ commandOrb' = [commandOrb EXCEPT ![c] = o]
  /\ commandResource' = [commandResource EXCEPT ![c] = r]
  /\ commandGeneration' = [commandGeneration EXCEPT
       ![c] = orbGeneration[o]]
  /\ commandPresence' = [commandPresence EXCEPT
       ![c] = presenceEpoch[a][o]]
  /\ commandPolicy' = [commandPolicy EXCEPT ![c] = policyEpoch[o]]
  /\ commandFence' = [commandFence EXCEPT ![c] = resourceFence[r]]
  /\ commandVersion' = [commandVersion EXCEPT ![c] = resourceVersion[r]]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion, staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts, effectOccurrences,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

CurrentAtCommit(c) ==
  LET a == commandAgent[c]
      o == commandOrb[c]
      r == commandResource[c]
  IN /\ a \in Agents
     /\ o \in Orbs
     /\ r \in Resources
     /\ Home(r) = o
     /\ PresenceCurrent(a, o)
     /\ RoleCanMutate(role[a][o])
     /\ resourceOwner[r] = a
     /\ commandGeneration[c] = orbGeneration[o]
     /\ commandPresence[c] = presenceEpoch[a][o]
     /\ commandPolicy[c] = policyEpoch[o]
     /\ commandFence[c] = resourceFence[r]
     /\ commandVersion[c] = resourceVersion[r]

Commit(c) ==
  /\ c \in Commands
  /\ commandStatus[c] = Issued
  /\ CurrentAtCommit(c)
  /\ commandStatus' = [commandStatus EXCEPT ![c] = Committed]
  /\ resourceVersion' = [resourceVersion EXCEPT
       ![commandResource[c]] = @ + 1]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence,
       commandAgent, commandOrb, commandResource, commandGeneration,
       commandPresence, commandPolicy, commandFence, commandVersion,
       staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts, effectOccurrences,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

RejectStale(c) ==
  /\ c \in Commands
  /\ commandStatus[c] = Issued
  /\ ~CurrentAtCommit(c)
  /\ commandStatus' = [commandStatus EXCEPT ![c] = Rejected]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandAgent, commandOrb, commandResource, commandGeneration,
       commandPresence, commandPolicy, commandFence, commandVersion,
       staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts, effectOccurrences,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

ReserveEffect(e, a, o) ==
  /\ e \in Effects
  /\ effectState[e] = EffectIdle
  /\ PresenceCurrent(a, o)
  /\ RoleCanEffect(role[a][o])
  /\ budgetAvailable[a] > 0
  /\ effectState' = [effectState EXCEPT ![e] = EffectReserved]
  /\ effectAgent' = [effectAgent EXCEPT ![e] = a]
  /\ effectOrb' = [effectOrb EXCEPT ![e] = o]
  /\ effectGeneration' = [effectGeneration EXCEPT
       ![e] = orbGeneration[o]]
  /\ effectPresence' = [effectPresence EXCEPT
       ![e] = presenceEpoch[a][o]]
  /\ effectPolicy' = [effectPolicy EXCEPT ![e] = policyEpoch[o]]
  /\ budgetAvailable' = [budgetAvailable EXCEPT ![a] = @ - 1]
  /\ effectEscrow' = [effectEscrow EXCEPT ![e] = 1]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, effectSpent>>

EffectSubmissionCurrent(e) ==
  LET a == effectAgent[e]
      o == effectOrb[e]
  IN /\ a \in Agents
     /\ o \in Orbs
     /\ PresenceCurrent(a, o)
     /\ RoleCanEffect(role[a][o])
     /\ effectGeneration[e] = orbGeneration[o]
     /\ effectPresence[e] = presenceEpoch[a][o]
     /\ effectPolicy[e] = policyEpoch[o]

SubmitEffect(e) ==
  /\ e \in Effects
  /\ effectState[e] = EffectReserved
  /\ EffectSubmissionCurrent(e)
  /\ effectAttempts[e] < MaxAttempts
  /\ IF effectAttempts[e] = 0
     THEN TRUE
     ELSE retryUses[e] < retryGrants[e]
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
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectAgent, effectOrb, effectGeneration, effectPresence, effectPolicy,
       effectOccurrences, retryGrants,
       budgetAvailable, effectEscrow, effectSpent>>

\* The broker may complete after presence or generation revocation. This action
\* deliberately has no EffectSubmissionCurrent guard. The idempotency key is e.
BrokerOccurs(e) ==
  /\ e \in Effects
  \* Delivery may still be in flight after the caller has classified the
  \* outcome Unknown. Reconciliation, not revocation or timeout, closes it.
  /\ effectState[e] \in {EffectSubmitted, EffectUnknown}
  /\ effectOccurrences[e] = 0
  /\ effectOccurrences' = [effectOccurrences EXCEPT ![e] = 1]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, effectAgent, effectOrb, effectGeneration,
       effectPresence, effectPolicy, effectAttempts,
       retryGrants, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

LoseOutcome(e) ==
  /\ e \in Effects
  /\ effectState[e] = EffectSubmitted
  /\ effectState' = [effectState EXCEPT ![e] = EffectUnknown]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectAgent, effectOrb, effectGeneration, effectPresence, effectPolicy,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable, effectEscrow, effectSpent>>

ConfirmOccurred(e) ==
  /\ e \in Effects
  /\ effectState[e] \in {EffectSubmitted, EffectUnknown}
  /\ effectOccurrences[e] = 1
  /\ effectState' = [effectState EXCEPT ![e] = EffectConfirmed]
  /\ effectEscrow' = [effectEscrow EXCEPT ![e] = 0]
  /\ effectSpent' = [effectSpent EXCEPT ![e] = 1]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectAgent, effectOrb, effectGeneration, effectPresence, effectPolicy,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, budgetAvailable>>

ReconcileNoOccurrence(e) ==
  /\ e \in Effects
  /\ effectState[e] = EffectUnknown
  /\ effectOccurrences[e] = 0
  /\ retryGrants[e] < MaxAttempts - 1
  /\ effectState' = [effectState EXCEPT ![e] = EffectReserved]
  /\ retryGrants' = [retryGrants EXCEPT ![e] = @ + 1]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectAgent, effectOrb, effectGeneration, effectPresence, effectPolicy,
       effectAttempts, effectOccurrences, retryUses, initialSubmitted,
       budgetAvailable, effectEscrow, effectSpent>>

CancelUnsubmitted(e) ==
  /\ e \in Effects
  /\ effectState[e] = EffectReserved
  /\ effectAttempts[e] = 0
  /\ LET a == effectAgent[e] IN a \in Agents
  /\ effectState' = [effectState EXCEPT ![e] = EffectCancelled]
  /\ budgetAvailable' = [budgetAvailable EXCEPT ![effectAgent[e]] = @ + 1]
  /\ effectEscrow' = [effectEscrow EXCEPT ![e] = 0]
  /\ UNCHANGED <<running, orbGeneration, policyEpoch, active, role,
       presenceEpoch, presenceStamp, presenceGeneration, presencePolicy,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandOrb, commandResource,
       commandGeneration, commandPresence, commandPolicy,
       commandFence, commandVersion, staleCommitAccepted,
       effectAgent, effectOrb, effectGeneration, effectPresence, effectPolicy,
       effectAttempts, effectOccurrences, retryGrants, retryUses,
       initialSubmitted, effectSpent>>

Next ==
  \/ \E a \in Agents, o \in Orbs, newRole \in Roles :
       SupervisorAdmit(a, o, newRole)
  \/ \E a \in Agents, o \in Orbs : SupervisorRevoke(a, o)
  \/ \E o \in Orbs : FailGeneration(o)
  \/ \E o \in Orbs : RecoverGeneration(o)
  \/ \E o \in Orbs : AdvancePolicy(o)
  \/ \E a \in Agents, o \in Orbs, r \in Resources : Acquire(a, o, r)
  \/ \E c \in Commands, a \in Agents, o \in Orbs, r \in Resources :
       Issue(c, a, o, r)
  \/ \E c \in Commands : Commit(c)
  \/ \E c \in Commands : RejectStale(c)
  \/ \E e \in Effects, a \in Agents, o \in Orbs : ReserveEffect(e, a, o)
  \/ \E e \in Effects : SubmitEffect(e)
  \/ \E e \in Effects : BrokerOccurs(e)
  \/ \E e \in Effects : LoseOutcome(e)
  \/ \E e \in Effects : ConfirmOccurred(e)
  \/ \E e \in Effects : ReconcileNoOccurrence(e)
  \/ \E e \in Effects : CancelUnsubmitted(e)

Spec == Init /\ [][Next]_vars

TypeOK ==
  /\ running \in [Orbs -> BOOLEAN]
  /\ orbGeneration \in [Orbs -> 0..MaxEpoch]
  /\ policyEpoch \in [Orbs -> 0..MaxEpoch]
  /\ active \in [Agents -> [Orbs -> BOOLEAN]]
  /\ role \in [Agents -> [Orbs -> Roles \cup {None}]]
  /\ presenceEpoch \in [Agents -> [Orbs -> 0..MaxEpoch]]
  /\ presenceStamp \in [Agents -> [Orbs -> 0..MaxEpoch]]
  /\ presenceGeneration \in [Agents -> [Orbs -> 0..MaxEpoch]]
  /\ presencePolicy \in [Agents -> [Orbs -> 0..MaxEpoch]]
  /\ resourceOwner \in [Resources -> Agents \cup {None}]
  /\ resourceFence \in [Resources -> Nat]
  /\ resourceVersion \in [Resources -> Nat]
  /\ commandStatus \in [Commands -> {Idle, Issued, Committed, Rejected}]
  /\ commandAgent \in [Commands -> Agents \cup {None}]
  /\ commandOrb \in [Commands -> Orbs \cup {None}]
  /\ commandResource \in [Commands -> Resources \cup {None}]
  /\ commandGeneration \in [Commands -> 0..MaxEpoch]
  /\ commandPresence \in [Commands -> 0..MaxEpoch]
  /\ commandPolicy \in [Commands -> 0..MaxEpoch]
  /\ commandFence \in [Commands -> Nat]
  /\ commandVersion \in [Commands -> Nat]
  /\ staleCommitAccepted \in BOOLEAN
  /\ effectState \in [Effects -> EffectStates]
  /\ effectAgent \in [Effects -> Agents \cup {None}]
  /\ effectOrb \in [Effects -> Orbs \cup {None}]
  /\ effectGeneration \in [Effects -> 0..MaxEpoch]
  /\ effectPresence \in [Effects -> 0..MaxEpoch]
  /\ effectPolicy \in [Effects -> 0..MaxEpoch]
  /\ effectAttempts \in [Effects -> 0..MaxAttempts]
  /\ effectOccurrences \in [Effects -> 0..1]
  /\ retryGrants \in [Effects -> 0..MaxAttempts]
  /\ retryUses \in [Effects -> 0..MaxAttempts]
  /\ initialSubmitted \in [Effects -> BOOLEAN]
  /\ budgetAvailable \in [Agents -> 0..MaxBudget]
  /\ effectEscrow \in [Effects -> 0..1]
  /\ effectSpent \in [Effects -> 0..1]

OwnerAuthorized ==
  \A r \in Resources :
    resourceOwner[r] = None \/
      LET a == resourceOwner[r]
          o == Home(r)
      IN /\ a \in Agents
         /\ o \in Orbs
         /\ PresenceCurrent(a, o)
         /\ RoleCanMutate(role[a][o])

NoStaleCommit == ~staleCommitAccepted

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

EffectWellFormed ==
  \A e \in Effects :
    /\ (effectState[e] = EffectIdle =>
          effectAgent[e] = None /\ effectEscrow[e] = 0 /\ effectSpent[e] = 0)
    /\ (effectState[e] # EffectIdle =>
          effectAgent[e] \in Agents /\ effectOrb[e] \in Orbs)

=============================================================================
