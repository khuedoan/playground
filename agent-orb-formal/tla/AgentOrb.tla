------------------------------ MODULE AgentOrb ------------------------------
EXTENDS Naturals, FiniteSets

CONSTANTS Agents, Resources, Commands, MaxBudget, MaxEpoch, SafeCommit

None == "none"
Idle == "idle"
Issued == "issued"
Committed == "committed"
Rejected == "rejected"

EffectAbsent == "effectAbsent"
EffectSubmitted == "effectSubmitted"
EffectUnknown == "effectUnknown"
EffectConfirmed == "effectConfirmed"

VARIABLES
  orbGeneration,
  policyEpoch,
  active,
  presenceEpoch,
  resourceOwner,
  resourceFence,
  resourceVersion,
  commandStatus,
  commandAgent,
  commandResource,
  commandOrbGeneration,
  commandPresenceEpoch,
  commandPolicyEpoch,
  commandFence,
  commandVersion,
  staleCommitAccepted,
  effectState,
  retryPermit,
  effectOwner,
  budgetAvailable,
  budgetEscrow,
  budgetSpent,
  causalFrontier

vars == <<
  orbGeneration,
  policyEpoch,
  active,
  presenceEpoch,
  resourceOwner,
  resourceFence,
  resourceVersion,
  commandStatus,
  commandAgent,
  commandResource,
  commandOrbGeneration,
  commandPresenceEpoch,
  commandPolicyEpoch,
  commandFence,
  commandVersion,
  staleCommitAccepted,
  effectState,
  retryPermit,
  effectOwner,
  budgetAvailable,
  budgetEscrow,
  budgetSpent,
  causalFrontier
>>

Init ==
  /\ orbGeneration = 0
  /\ policyEpoch = 0
  /\ active = {}
  /\ presenceEpoch = [a \in Agents |-> 0]
  /\ resourceOwner = [r \in Resources |-> None]
  /\ resourceFence = [r \in Resources |-> 0]
  /\ resourceVersion = [r \in Resources |-> 0]
  /\ commandStatus = [c \in Commands |-> Idle]
  /\ commandAgent = [c \in Commands |-> None]
  /\ commandResource = [c \in Commands |-> None]
  /\ commandOrbGeneration = [c \in Commands |-> 0]
  /\ commandPresenceEpoch = [c \in Commands |-> 0]
  /\ commandPolicyEpoch = [c \in Commands |-> 0]
  /\ commandFence = [c \in Commands |-> 0]
  /\ commandVersion = [c \in Commands |-> 0]
  /\ staleCommitAccepted = FALSE
  /\ effectState = EffectAbsent
  /\ retryPermit = FALSE
  /\ effectOwner = None
  /\ budgetAvailable = [a \in Agents |-> MaxBudget]
  /\ budgetEscrow = [a \in Agents |-> 0]
  /\ budgetSpent = [a \in Agents |-> 0]
  /\ causalFrontier = 0

SupervisorJoin(a) ==
  /\ a \in Agents
  /\ a \notin active
  /\ active' = active \cup {a}
  /\ UNCHANGED <<orbGeneration, policyEpoch, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandResource,
       commandOrbGeneration, commandPresenceEpoch, commandPolicyEpoch,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

Revoke(a) ==
  /\ a \in active
  /\ presenceEpoch[a] < MaxEpoch
  /\ active' = active \ {a}
  /\ presenceEpoch' = [presenceEpoch EXCEPT ![a] = @ + 1]
  /\ resourceOwner' = [r \in Resources |->
       IF resourceOwner[r] = a THEN None ELSE resourceOwner[r]]
  /\ resourceFence' = [r \in Resources |->
       IF resourceOwner[r] = a THEN resourceFence[r] + 1 ELSE resourceFence[r]]
  /\ UNCHANGED <<orbGeneration, policyEpoch, resourceVersion,
       commandStatus, commandAgent, commandResource,
       commandOrbGeneration, commandPresenceEpoch, commandPolicyEpoch,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

AdvanceGeneration ==
  /\ orbGeneration < MaxEpoch
  /\ \A a \in Agents : presenceEpoch[a] < MaxEpoch
  /\ orbGeneration' = orbGeneration + 1
  /\ active' = {}
  /\ presenceEpoch' = [a \in Agents |-> presenceEpoch[a] + 1]
  /\ resourceOwner' = [r \in Resources |-> None]
  /\ resourceFence' = [r \in Resources |-> resourceFence[r] + 1]
  /\ UNCHANGED <<policyEpoch, resourceVersion,
       commandStatus, commandAgent, commandResource,
       commandOrbGeneration, commandPresenceEpoch, commandPolicyEpoch,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

AdvancePolicy ==
  /\ policyEpoch < MaxEpoch
  /\ policyEpoch' = policyEpoch + 1
  /\ resourceOwner' = [r \in Resources |-> None]
  /\ resourceFence' = [r \in Resources |-> resourceFence[r] + 1]
  /\ UNCHANGED <<orbGeneration, active, presenceEpoch, resourceVersion,
       commandStatus, commandAgent, commandResource,
       commandOrbGeneration, commandPresenceEpoch, commandPolicyEpoch,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

Acquire(a, r) ==
  /\ a \in active
  /\ r \in Resources
  /\ resourceOwner[r] = None
  /\ resourceOwner' = [resourceOwner EXCEPT ![r] = a]
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceFence, resourceVersion,
       commandStatus, commandAgent, commandResource,
       commandOrbGeneration, commandPresenceEpoch, commandPolicyEpoch,
       commandFence, commandVersion, staleCommitAccepted,
       effectState, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

Issue(c, a, r) ==
  /\ c \in Commands
  /\ a \in active
  /\ r \in Resources
  /\ commandStatus[c] = Idle
  /\ resourceOwner[r] = a
  /\ commandStatus' = [commandStatus EXCEPT ![c] = Issued]
  /\ commandAgent' = [commandAgent EXCEPT ![c] = a]
  /\ commandResource' = [commandResource EXCEPT ![c] = r]
  /\ commandOrbGeneration' = [commandOrbGeneration EXCEPT ![c] = orbGeneration]
  /\ commandPresenceEpoch' = [commandPresenceEpoch EXCEPT ![c] = presenceEpoch[a]]
  /\ commandPolicyEpoch' = [commandPolicyEpoch EXCEPT ![c] = policyEpoch]
  /\ commandFence' = [commandFence EXCEPT ![c] = resourceFence[r]]
  /\ commandVersion' = [commandVersion EXCEPT ![c] = resourceVersion[r]]
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion, staleCommitAccepted,
       effectState, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

CurrentAtCommit(c) ==
  LET a == commandAgent[c]
      r == commandResource[c]
  IN /\ a \in active
     /\ r \in Resources
     /\ resourceOwner[r] = a
     /\ commandOrbGeneration[c] = orbGeneration
     /\ commandPresenceEpoch[c] = presenceEpoch[a]
     /\ commandPolicyEpoch[c] = policyEpoch
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
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence,
       commandAgent, commandResource, commandOrbGeneration,
       commandPresenceEpoch, commandPolicyEpoch, commandFence, commandVersion,
       effectState, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

RejectStale(c) ==
  /\ c \in Commands
  /\ commandStatus[c] = Issued
  /\ ~CurrentAtCommit(c)
  /\ commandStatus' = [commandStatus EXCEPT ![c] = Rejected]
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandAgent, commandResource, commandOrbGeneration,
       commandPresenceEpoch, commandPolicyEpoch, commandFence, commandVersion,
       staleCommitAccepted, effectState, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

ReserveBudget(a) ==
  /\ a \in active
  /\ budgetAvailable[a] > 0
  /\ budgetAvailable' = [budgetAvailable EXCEPT ![a] = @ - 1]
  /\ budgetEscrow' = [budgetEscrow EXCEPT ![a] = @ + 1]
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandResource, commandOrbGeneration,
       commandPresenceEpoch, commandPolicyEpoch, commandFence, commandVersion,
       staleCommitAccepted, effectState, retryPermit, effectOwner,
       budgetSpent, causalFrontier>>

SettleBudget(a) ==
  /\ budgetEscrow[a] > 0
  /\ budgetEscrow' = [budgetEscrow EXCEPT ![a] = @ - 1]
  /\ budgetSpent' = [budgetSpent EXCEPT ![a] = @ + 1]
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandResource, commandOrbGeneration,
       commandPresenceEpoch, commandPolicyEpoch, commandFence, commandVersion,
       staleCommitAccepted, effectState, retryPermit, effectOwner,
       budgetAvailable, causalFrontier>>

SubmitEffect(a) ==
  /\ a \in active
  /\ effectState = EffectAbsent
  /\ budgetEscrow[a] > 0
  /\ effectState' = EffectSubmitted
  /\ effectOwner' = a
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandResource, commandOrbGeneration,
       commandPresenceEpoch, commandPolicyEpoch, commandFence, commandVersion,
       staleCommitAccepted, retryPermit,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

RecordOutcome(outcome) ==
  /\ effectState = EffectSubmitted
  /\ outcome \in {EffectUnknown, EffectConfirmed}
  /\ effectState' = outcome
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandResource, commandOrbGeneration,
       commandPresenceEpoch, commandPolicyEpoch, commandFence, commandVersion,
       staleCommitAccepted, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

GrantRetry ==
  /\ effectState = EffectUnknown
  /\ retryPermit' = TRUE
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandResource, commandOrbGeneration,
       commandPresenceEpoch, commandPolicyEpoch, commandFence, commandVersion,
       staleCommitAccepted, effectState, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

RetryEffect ==
  /\ effectState = EffectUnknown
  /\ retryPermit
  /\ effectState' = EffectSubmitted
  /\ retryPermit' = FALSE
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandResource, commandOrbGeneration,
       commandPresenceEpoch, commandPolicyEpoch, commandFence, commandVersion,
       staleCommitAccepted, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent, causalFrontier>>

AdvanceFrontier ==
  /\ causalFrontier < MaxEpoch
  /\ causalFrontier' = causalFrontier + 1
  /\ UNCHANGED <<orbGeneration, policyEpoch, active, presenceEpoch,
       resourceOwner, resourceFence, resourceVersion,
       commandStatus, commandAgent, commandResource, commandOrbGeneration,
       commandPresenceEpoch, commandPolicyEpoch, commandFence, commandVersion,
       staleCommitAccepted, effectState, retryPermit, effectOwner,
       budgetAvailable, budgetEscrow, budgetSpent>>

Next ==
  \/ \E a \in Agents : SupervisorJoin(a)
  \/ \E a \in Agents : Revoke(a)
  \/ AdvanceGeneration
  \/ AdvancePolicy
  \/ \E a \in Agents, r \in Resources : Acquire(a, r)
  \/ \E c \in Commands, a \in Agents, r \in Resources : Issue(c, a, r)
  \/ \E c \in Commands : Commit(c)
  \/ \E c \in Commands : RejectStale(c)
  \/ \E a \in Agents : ReserveBudget(a)
  \/ \E a \in Agents : SettleBudget(a)
  \/ \E a \in Agents : SubmitEffect(a)
  \/ \E outcome \in {EffectUnknown, EffectConfirmed} : RecordOutcome(outcome)
  \/ GrantRetry
  \/ RetryEffect
  \/ AdvanceFrontier

Spec == Init /\ [][Next]_vars

TypeOK ==
  /\ orbGeneration \in Nat
  /\ policyEpoch \in Nat
  /\ active \subseteq Agents
  /\ presenceEpoch \in [Agents -> Nat]
  /\ resourceOwner \in [Resources -> Agents \cup {None}]
  /\ resourceFence \in [Resources -> Nat]
  /\ resourceVersion \in [Resources -> Nat]
  /\ commandStatus \in [Commands -> {Idle, Issued, Committed, Rejected}]
  /\ commandAgent \in [Commands -> Agents \cup {None}]
  /\ commandResource \in [Commands -> Resources \cup {None}]
  /\ budgetAvailable \in [Agents -> Nat]
  /\ budgetEscrow \in [Agents -> Nat]
  /\ budgetSpent \in [Agents -> Nat]

OwnerIsActive ==
  \A r \in Resources : resourceOwner[r] = None \/ resourceOwner[r] \in active

NoStaleCommit == ~staleCommitAccepted

BudgetConserved ==
  \A a \in Agents :
    budgetAvailable[a] + budgetEscrow[a] + budgetSpent[a] = MaxBudget

NoAutomaticRetry == effectState # EffectSubmitted \/ ~retryPermit

=============================================================================
