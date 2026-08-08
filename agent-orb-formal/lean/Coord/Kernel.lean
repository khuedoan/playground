import Std.Tactic

namespace AgentOrb

abbrev OrbId := Nat
abbrev AgentId := Nat
abbrev ResourceId := Nat
abbrev EnvironmentId := Nat

inductive Right where
  | observe
  | operate
  | retryEffect
  | administer
deriving DecidableEq

structure Rights where
  allows : Right → Bool

def Rights.Subset (child parent : Rights) : Prop :=
  ∀ right, child.allows right = true → parent.allows right = true

def Rights.attenuate (parent requested : Rights) : Rights :=
  ⟨fun right => parent.allows right && requested.allows right⟩

theorem Rights.attenuate_no_escalation (parent requested : Rights) :
    (Rights.attenuate parent requested).Subset parent := by
  intro right allowed
  simp [Rights.attenuate] at allowed
  exact allowed.1

structure OrbState where
  id : OrbId
  generation : Nat
  policyEpoch : Nat
deriving DecidableEq

structure OwnerSessionState where
  /-- The permanent owner of this orb. This field is never reassigned. -/
  agentId : AgentId
  active : Bool
  ownerEpoch : Nat
deriving DecidableEq

structure EnvironmentState where
  id : EnvironmentId
  connected : Bool
  epoch : Nat
deriving DecidableEq

structure ResourceState where
  id : ResourceId
  generation : Nat
  fence : Nat
  version : Nat
  owner : Option AgentId

structure BudgetState where
  available : Nat
  reserved : Nat
  spent : Nat
  escrowFence : Nat
deriving DecidableEq

inductive EffectState where
  | absent
  | submitted (idempotencyKey attempt : Nat)
  | unknown (idempotencyKey attempt : Nat) (explicitRetryPermit : Bool)
  | confirmed (idempotencyKey attempt : Nat)
deriving DecidableEq

structure Snapshot where
  /-- CAS revision for one resource/escrow actor shard, never a system-global lock. -/
  shardRevision : Nat
  orb : OrbState
  ownerSession : OwnerSessionState
  resource : ResourceState
  environment : EnvironmentState
  /-- A monotone delivery cursor; this is not a proof of causal consistency. -/
  observedSequence : Nat
  committedSequence : Nat
  logicalTime : Nat
  budget : BudgetState
  effect : EffectState
  effectFence : Nat

def OwnerSafe (s : Snapshot) : Prop :=
  match s.resource.owner with
  | none => True
  | some owner => owner = s.ownerSession.agentId ∧ s.ownerSession.active = true

def Valid (s : Snapshot) : Prop :=
  s.resource.generation = s.orb.generation ∧
  OwnerSafe s ∧
  s.observedSequence ≤ s.committedSequence

structure Token where
  orbId : OrbId
  orbGeneration : Nat
  policyEpoch : Nat
  agentId : AgentId
  ownerEpoch : Nat
  resourceId : ResourceId
  resourceFence : Nat
  visibleThrough : Nat
  expiresAt : Nat
  maxReservation : Nat
  rights : Rights

/-!
Trust boundary: `Token`, `EnvironmentToken`, `AdminToken`, `BrokerReceipt`, and `EscrowReceipt` are
logical projections of credentials already authenticated by the gateway.  This
kernel proves admission from their fields; it does not prove cryptography,
credential minting, durable CAS, or that every production path invokes `step`.
-/

/-- Restrict a bearer for the same owner. It cannot change the agent identity. -/
def Token.restrict (parent : Token) (requested : Rights)
    (visibleThrough expiresAt maxReservation : Nat) : Token :=
  { parent with
    visibleThrough := min parent.visibleThrough visibleThrough
    expiresAt := min parent.expiresAt expiresAt
    maxReservation := min parent.maxReservation maxReservation
    rights := parent.rights.attenuate requested }

def Token.Attenuates (child parent : Token) : Prop :=
  child.orbId = parent.orbId ∧
  child.orbGeneration = parent.orbGeneration ∧
  child.policyEpoch = parent.policyEpoch ∧
  child.agentId = parent.agentId ∧
  child.ownerEpoch = parent.ownerEpoch ∧
  child.resourceId = parent.resourceId ∧
  child.resourceFence = parent.resourceFence ∧
  child.visibleThrough ≤ parent.visibleThrough ∧
  child.expiresAt ≤ parent.expiresAt ∧
  child.maxReservation ≤ parent.maxReservation ∧
  child.rights.Subset parent.rights

theorem Token.restrict_sound (parent : Token) (requested : Rights)
    (visibleThrough expiresAt maxReservation : Nat) :
    (parent.restrict requested visibleThrough expiresAt maxReservation).Attenuates parent := by
  refine ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, ?_, ?_, ?_, ?_⟩
  · exact Nat.min_le_left _ _
  · exact Nat.min_le_left _ _
  · exact Nat.min_le_left _ _
  · exact Rights.attenuate_no_escalation _ _

structure AdminToken where
  orbId : OrbId
  orbGeneration : Nat
  policyEpoch : Nat
  rights : Rights

/-- A separate capability for a shared service. It grants no access to another orb. -/
structure EnvironmentToken where
  orbId : OrbId
  orbGeneration : Nat
  policyEpoch : Nat
  agentId : AgentId
  ownerEpoch : Nat
  environmentId : EnvironmentId
  environmentEpoch : Nat
  expiresAt : Nat

def AdminCurrent (s : Snapshot) (token : AdminToken) : Prop :=
  token.orbId = s.orb.id ∧
  token.orbGeneration = s.orb.generation ∧
  token.policyEpoch = s.orb.policyEpoch ∧
  token.rights.allows .administer = true

instance (s : Snapshot) (token : AdminToken) : Decidable (AdminCurrent s token) := by
  unfold AdminCurrent
  infer_instance

structure BrokerReceipt where
  orbId : OrbId
  effectFence : Nat
  idempotencyKey : Nat
  attempt : Nat

def BrokerCurrent (s : Snapshot) (receipt : BrokerReceipt) : Prop :=
  receipt.orbId = s.orb.id ∧ receipt.effectFence = s.effectFence

instance (s : Snapshot) (receipt : BrokerReceipt) : Decidable (BrokerCurrent s receipt) := by
  unfold BrokerCurrent
  infer_instance

structure EscrowReceipt where
  orbId : OrbId
  escrowFence : Nat

def EscrowCurrent (s : Snapshot) (receipt : EscrowReceipt) : Prop :=
  receipt.orbId = s.orb.id ∧ receipt.escrowFence = s.budget.escrowFence

instance (s : Snapshot) (receipt : EscrowReceipt) : Decidable (EscrowCurrent s receipt) := by
  unfold EscrowCurrent
  infer_instance

/-- All volatile authority dimensions are checked together at commit time. -/
def Current (s : Snapshot) (token : Token) : Prop :=
  token.orbId = s.orb.id ∧
  token.orbGeneration = s.orb.generation ∧
  token.policyEpoch = s.orb.policyEpoch ∧
  token.agentId = s.ownerSession.agentId ∧
  token.ownerEpoch = s.ownerSession.ownerEpoch ∧
  s.ownerSession.active = true ∧
  token.resourceId = s.resource.id ∧
  token.resourceFence = s.resource.fence ∧
  s.logicalTime < token.expiresAt

def EnvironmentCurrent (s : Snapshot) (token : EnvironmentToken) : Prop :=
  token.orbId = s.orb.id ∧
  token.orbGeneration = s.orb.generation ∧
  token.policyEpoch = s.orb.policyEpoch ∧
  token.agentId = s.ownerSession.agentId ∧
  token.ownerEpoch = s.ownerSession.ownerEpoch ∧
  s.ownerSession.active = true ∧
  token.environmentId = s.environment.id ∧
  token.environmentEpoch = s.environment.epoch ∧
  s.environment.connected = true ∧
  s.logicalTime < token.expiresAt

instance (s : Snapshot) (token : EnvironmentToken) :
    Decidable (EnvironmentCurrent s token) := by
  unfold EnvironmentCurrent
  infer_instance

instance (s : Snapshot) (token : Token) : Decidable (Current s token) := by
  unfold Current
  infer_instance

def current? (s : Snapshot) (token : Token) : Bool := decide (Current s token)

theorem current?_eq_true_iff (s : Snapshot) (token : Token) :
    current? s token = true ↔ Current s token := by
  simp [current?]

def Authorized (s : Snapshot) (token : Token) (right : Right) : Prop :=
  Current s token ∧ token.rights.allows right = true

instance (s : Snapshot) (token : Token) (right : Right) :
    Decidable (Authorized s token right) := by
  unfold Authorized
  infer_instance

def authorized? (s : Snapshot) (token : Token) (right : Right) : Bool :=
  decide (Authorized s token right)

theorem authorized?_eq_true_iff (s : Snapshot) (token : Token) (right : Right) :
    authorized? s token right = true ↔ Authorized s token right := by
  simp [authorized?]

/-- Agent communication is data-only. It deliberately has no orb or capability field. -/
structure AgentMessage where
  fromAgent : AgentId
  toAgent : AgentId
  payloadHash : Nat
deriving DecidableEq

inductive Reject where
  | staleRevision
  | unauthorized
  | resourceBusy
  | staleResourceVersion
  | insufficientBudget
  | invalidEffectState
  | staleSystemEpoch
deriving DecidableEq

inductive Event where
  | ownerSessionStarted
  | ownerSessionRevoked
  | generationAdvanced
  | policyAdvanced
  | environmentConnected
  | environmentDisconnected
  | leaseAcquired
  | leaseReleased
  | mutationCommitted
  | frontierAdvanced
  | budgetReserved
  | budgetSettled
  | budgetRefunded
  | effectSubmitted
  | effectUnknown
  | retryGranted
  | effectRetried
  | effectConfirmed
  | timeAdvanced
  | sequenceCommitted
deriving DecidableEq

inductive Command where
  | startOwnerSession (admin : AdminToken) (expectedOwnerEpoch : Nat)
  | revokeOwnerSession (admin : AdminToken) (expectedOwnerEpoch : Nat)
  | advanceGeneration (admin : AdminToken)
  | advancePolicy (admin : AdminToken)
  | connectEnvironment (admin : AdminToken)
  | disconnectEnvironment (admin : AdminToken) (expectedEnvironmentEpoch : Nat)
  | advanceTime (admin : AdminToken) (next : Nat)
  | commitSequence (admin : AdminToken) (next : Nat)
  | acquireExclusive (token : Token)
  | releaseExclusive (token : Token)
  | commitMutation (token : Token) (expectedResourceVersion : Nat)
  | advanceObservedSequence (token : Token) (next : Nat)
  | reserveBudget (token : Token) (amount : Nat)
  | settleBudget (receipt : EscrowReceipt)
  | refundBudget (receipt : EscrowReceipt)
  | submitEffect (token : Token) (environmentToken : EnvironmentToken)
      (idempotencyKey : Nat)
  | markEffectUnknown (receipt : BrokerReceipt)
  | grantExplicitRetry (admin : AdminToken) (expectedAttempt : Nat)
  | retryEffect (token : Token) (environmentToken : EnvironmentToken)
  | confirmEffect (receipt : BrokerReceipt)

structure Envelope where
  expectedShardRevision : Nat
  command : Command

structure Applied where
  state : Snapshot
  event : Event

def applyCommand (s : Snapshot) : Command → Except Reject Applied
  | .startOwnerSession admin epoch =>
      if AdminCurrent s admin ∧ epoch = s.ownerSession.ownerEpoch then
        let ownerSession := { s.ownerSession with active := true }
        .ok ⟨{ s with ownerSession }, .ownerSessionStarted⟩
      else .error .staleSystemEpoch
  | .revokeOwnerSession admin epoch =>
      if AdminCurrent s admin ∧ epoch = s.ownerSession.ownerEpoch then
        let ownerSession := { s.ownerSession with
          active := false
          ownerEpoch := s.ownerSession.ownerEpoch + 1 }
        let resource := { s.resource with
          owner := none
          fence := s.resource.fence + 1 }
        .ok ⟨{ s with ownerSession, resource }, .ownerSessionRevoked⟩
      else .error .staleSystemEpoch
  | .advanceGeneration admin =>
      if AdminCurrent s admin then
        let nextGeneration := s.orb.generation + 1
        let orb := { s.orb with generation := nextGeneration }
        let ownerSession := { s.ownerSession with
          active := false
          ownerEpoch := s.ownerSession.ownerEpoch + 1 }
        let resource := { s.resource with
          generation := nextGeneration
          owner := none
          fence := s.resource.fence + 1 }
        .ok ⟨{ s with orb, ownerSession, resource }, .generationAdvanced⟩
      else .error .staleSystemEpoch
  | .advancePolicy admin =>
      if AdminCurrent s admin then
        let orb := { s.orb with policyEpoch := s.orb.policyEpoch + 1 }
        let resource := { s.resource with
          owner := none
          fence := s.resource.fence + 1 }
        .ok ⟨{ s with orb, resource }, .policyAdvanced⟩
      else .error .staleSystemEpoch
  | .connectEnvironment admin =>
      if AdminCurrent s admin then
        let environment := { s.environment with connected := true }
        .ok ⟨{ s with environment }, .environmentConnected⟩
      else .error .staleSystemEpoch
  | .disconnectEnvironment admin epoch =>
      if AdminCurrent s admin ∧ epoch = s.environment.epoch then
        let environment := { s.environment with
          connected := false
          epoch := s.environment.epoch + 1 }
        .ok ⟨{ s with environment }, .environmentDisconnected⟩
      else .error .staleSystemEpoch
  | .advanceTime admin next =>
      if AdminCurrent s admin ∧ s.logicalTime ≤ next then
        .ok ⟨{ s with logicalTime := next }, .timeAdvanced⟩
      else .error .staleSystemEpoch
  | .commitSequence admin next =>
      if AdminCurrent s admin ∧ s.committedSequence ≤ next then
        .ok ⟨{ s with committedSequence := next }, .sequenceCommitted⟩
      else .error .staleSystemEpoch
  | .acquireExclusive token =>
      if Authorized s token .operate then
        match s.resource.owner with
        | none =>
            let resource := { s.resource with owner := some token.agentId }
            .ok ⟨{ s with resource }, .leaseAcquired⟩
        | some _ => .error .resourceBusy
      else .error .unauthorized
  | .releaseExclusive token =>
      if Authorized s token .operate then
        match s.resource.owner with
        | some owner =>
            if owner = token.agentId then
              let resource := { s.resource with
                owner := none
                fence := s.resource.fence + 1 }
              .ok ⟨{ s with resource }, .leaseReleased⟩
            else .error .resourceBusy
        | none => .error .resourceBusy
      else .error .unauthorized
  | .commitMutation token expectedVersion =>
      if Authorized s token .operate then
        match s.resource.owner with
        | some owner =>
            if owner = token.agentId then
              if expectedVersion = s.resource.version then
                let resource := { s.resource with version := s.resource.version + 1 }
                .ok ⟨{ s with resource }, .mutationCommitted⟩
              else .error .staleResourceVersion
            else .error .resourceBusy
        | none => .error .resourceBusy
      else .error .unauthorized
  | .advanceObservedSequence token next =>
      if Authorized s token .observe then
        if s.observedSequence ≤ next ∧
            next ≤ s.committedSequence ∧ next ≤ token.visibleThrough then
          .ok ⟨{ s with observedSequence := next }, .frontierAdvanced⟩
        else .error .staleResourceVersion
      else .error .unauthorized
  | .reserveBudget token amount =>
      if Authorized s token .operate then
        if s.budget.reserved = 0 ∧
            amount ≤ token.maxReservation ∧ amount ≤ s.budget.available then
          let nextFence := s.budget.escrowFence + 1
          let budget := { s.budget with
            available := s.budget.available - amount
            reserved := amount
            escrowFence := nextFence }
          .ok ⟨{ s with budget }, .budgetReserved⟩
        else .error .insufficientBudget
      else .error .unauthorized
  | .settleBudget receipt =>
      if EscrowCurrent s receipt ∧ 0 < s.budget.reserved then
        let budget := { s.budget with
          reserved := 0
          spent := s.budget.spent + s.budget.reserved
          escrowFence := s.budget.escrowFence + 1 }
        .ok ⟨{ s with budget }, .budgetSettled⟩
      else .error .unauthorized
  | .refundBudget receipt =>
      if EscrowCurrent s receipt ∧ 0 < s.budget.reserved then
        let budget := { s.budget with
          reserved := 0
          available := s.budget.available + s.budget.reserved
          escrowFence := s.budget.escrowFence + 1 }
        .ok ⟨{ s with budget }, .budgetRefunded⟩
      else .error .unauthorized
  | .submitEffect token environmentToken idempotencyKey =>
      if Authorized s token .operate ∧ EnvironmentCurrent s environmentToken ∧
          environmentToken.agentId = token.agentId then
        match s.effect with
        | .absent =>
            let nextFence := s.effectFence + 1
            let stateWithEffect := { s with effect := (.submitted idempotencyKey nextFence) }
            .ok ⟨{ stateWithEffect with effectFence := nextFence },
              .effectSubmitted⟩
        | _ => .error .invalidEffectState
      else .error .unauthorized
  | .markEffectUnknown receipt =>
      if BrokerCurrent s receipt then
        match s.effect with
        | .submitted key attempt =>
            if receipt.idempotencyKey = key ∧ receipt.attempt = attempt then
              .ok ⟨{ s with effect := (.unknown key attempt false) }, .effectUnknown⟩
            else .error .invalidEffectState
        | _ => .error .invalidEffectState
      else .error .unauthorized
  | .grantExplicitRetry admin expectedAttempt =>
      if AdminCurrent s admin then
        match s.effect with
        | .unknown key attempt _ =>
            if expectedAttempt = attempt then
              .ok ⟨{ s with effect := (.unknown key attempt true) }, .retryGranted⟩
            else .error .invalidEffectState
        | _ => .error .invalidEffectState
      else .error .unauthorized
  | .retryEffect token environmentToken =>
      if Authorized s token .retryEffect ∧ EnvironmentCurrent s environmentToken ∧
          environmentToken.agentId = token.agentId then
        match s.effect with
        | .unknown key _ true =>
            let nextFence := s.effectFence + 1
            let stateWithEffect := { s with effect := (.submitted key nextFence) }
            .ok ⟨{ stateWithEffect with effectFence := nextFence },
              .effectRetried⟩
        | _ => .error .invalidEffectState
      else .error .unauthorized
  | .confirmEffect receipt =>
      if BrokerCurrent s receipt then
        match s.effect with
        | .submitted key attempt =>
            if receipt.idempotencyKey = key ∧ receipt.attempt = attempt then
              .ok ⟨{ s with effect := (.confirmed key attempt) }, .effectConfirmed⟩
            else .error .invalidEffectState
        | .unknown key attempt _ =>
            if receipt.idempotencyKey = key ∧ receipt.attempt = attempt then
              .ok ⟨{ s with effect := (.confirmed key attempt) }, .effectConfirmed⟩
            else .error .invalidEffectState
        | _ => .error .invalidEffectState
      else .error .unauthorized

/--
The owning actor/shard CAS serializes admission for this resource and its local escrow.
Different resources and budget escrows use independent shards and revisions.
-/
def step (s : Snapshot) (envelope : Envelope) : Except Reject Applied :=
  if envelope.expectedShardRevision = s.shardRevision then
    match applyCommand s envelope.command with
    | .error reject => .error reject
    | .ok applied =>
        .ok { applied with state := { applied.state with shardRevision := s.shardRevision + 1 } }
  else .error .staleRevision

def BudgetTotal (budget : BudgetState) : Nat :=
  budget.available + budget.reserved + budget.spent

theorem exclusive_owner_unique (s : Snapshot) {first second : AgentId}
    (hfirst : s.resource.owner = some first)
    (hsecond : s.resource.owner = some second) : first = second := by
  rw [hfirst] at hsecond
  exact Option.some.inj hsecond

theorem acquire_success_authorized {s : Snapshot} {token : Token} {applied : Applied}
    (success : applyCommand s (.acquireExclusive token) = .ok applied) :
    Authorized s token .operate := by
  grind [applyCommand]

theorem acquire_requires_unowned {s : Snapshot} {token : Token} {applied : Applied}
    (success : applyCommand s (.acquireExclusive token) = .ok applied) :
    s.resource.owner = none := by
  grind [applyCommand]

theorem acquire_sets_unique_owner {s : Snapshot} {token : Token} {applied : Applied}
    (success : applyCommand s (.acquireExclusive token) = .ok applied) :
    applied.state.resource.owner = some token.agentId := by
  grind [applyCommand]

theorem commit_success_current {s : Snapshot} {token : Token} {version : Nat}
    {applied : Applied}
    (success : applyCommand s (.commitMutation token version) = .ok applied) :
    Current s token := by
  grind [applyCommand, Authorized]

theorem commit_success_fenced_owner {s : Snapshot} {token : Token} {version : Nat}
    {applied : Applied}
    (success : applyCommand s (.commitMutation token version) = .ok applied) :
    s.resource.owner = some token.agentId := by
  grind [applyCommand]

theorem commit_success_resource_cas {s : Snapshot} {token : Token} {version : Nat}
    {applied : Applied}
    (success : applyCommand s (.commitMutation token version) = .ok applied) :
    version = s.resource.version := by
  grind [applyCommand]

theorem stale_generation_rejected {s : Snapshot} {token : Token} {version : Nat}
    {applied : Applied} (stale : token.orbGeneration ≠ s.orb.generation) :
    applyCommand s (.commitMutation token version) ≠ .ok applied := by
  grind [applyCommand, Authorized, Current]

theorem stale_owner_epoch_rejected {s : Snapshot} {token : Token} {version : Nat}
    {applied : Applied} (stale : token.ownerEpoch ≠ s.ownerSession.ownerEpoch) :
    applyCommand s (.commitMutation token version) ≠ .ok applied := by
  grind [applyCommand, Authorized, Current]

theorem stale_policy_rejected {s : Snapshot} {token : Token} {version : Nat}
    {applied : Applied} (stale : token.policyEpoch ≠ s.orb.policyEpoch) :
    applyCommand s (.commitMutation token version) ≠ .ok applied := by
  grind [applyCommand, Authorized, Current]

theorem stale_fence_rejected {s : Snapshot} {token : Token} {version : Nat}
    {applied : Applied} (stale : token.resourceFence ≠ s.resource.fence) :
    applyCommand s (.commitMutation token version) ≠ .ok applied := by
  grind [applyCommand, Authorized, Current]

theorem expired_token_rejected {s : Snapshot} {token : Token} {version : Nat}
    {applied : Applied} (expired : token.expiresAt ≤ s.logicalTime) :
    applyCommand s (.commitMutation token version) ≠ .ok applied := by
  grind [applyCommand, Authorized, Current]

theorem observed_sequence_monotone_and_bounded {s : Snapshot} {token : Token}
    {next : Nat} {applied : Applied}
    (success : applyCommand s (.advanceObservedSequence token next) = .ok applied) :
    s.observedSequence ≤ applied.state.observedSequence ∧
    applied.state.observedSequence ≤ s.committedSequence ∧
    applied.state.observedSequence ≤ token.visibleThrough := by
  grind [applyCommand]

theorem reserve_success_is_escrowed {s : Snapshot} {token : Token} {amount : Nat}
    {applied : Applied}
    (success : applyCommand s (.reserveBudget token amount) = .ok applied) :
    s.budget.reserved = 0 ∧
    amount ≤ token.maxReservation ∧
    amount ≤ s.budget.available ∧
    applied.state.budget.reserved = amount ∧
    applied.state.budget.escrowFence = s.budget.escrowFence + 1 := by
  grind [applyCommand]

theorem reserve_preserves_budget_total {s : Snapshot} {token : Token} {amount : Nat}
    {applied : Applied}
    (success : applyCommand s (.reserveBudget token amount) = .ok applied) :
    BudgetTotal applied.state.budget = BudgetTotal s.budget := by
  grind [applyCommand, BudgetTotal]

theorem settle_requires_current_escrow {s : Snapshot} {receipt : EscrowReceipt}
    {applied : Applied}
    (success : applyCommand s (.settleBudget receipt) = .ok applied) :
    EscrowCurrent s receipt := by
  grind [applyCommand]

theorem settle_consumes_and_fences {s : Snapshot} {receipt : EscrowReceipt}
    {applied : Applied}
    (success : applyCommand s (.settleBudget receipt) = .ok applied) :
    applied.state.budget.reserved = 0 ∧
    applied.state.budget.escrowFence = s.budget.escrowFence + 1 ∧
    BudgetTotal applied.state.budget = BudgetTotal s.budget := by
  grind [applyCommand, BudgetTotal]

theorem settled_receipt_cannot_repeat {s : Snapshot} {receipt : EscrowReceipt}
    {applied repeated : Applied}
    (success : applyCommand s (.settleBudget receipt) = .ok applied) :
    applyCommand applied.state (.settleBudget receipt) ≠ .ok repeated := by
  grind [applyCommand, EscrowCurrent]

theorem refund_consumes_and_fences {s : Snapshot} {receipt : EscrowReceipt}
    {applied : Applied}
    (success : applyCommand s (.refundBudget receipt) = .ok applied) :
    applied.state.budget.reserved = 0 ∧
    applied.state.budget.escrowFence = s.budget.escrowFence + 1 ∧
    BudgetTotal applied.state.budget = BudgetTotal s.budget := by
  grind [applyCommand, BudgetTotal]

theorem retry_success_requires_explicit_gate {s : Snapshot} {token : Token}
    {environmentToken : EnvironmentToken} {applied : Applied}
    (success : applyCommand s (.retryEffect token environmentToken) = .ok applied) :
    Authorized s token .retryEffect ∧
    EnvironmentCurrent s environmentToken ∧
    environmentToken.agentId = token.agentId ∧
    ∃ key attempt, s.effect = .unknown key attempt true ∧
      applied.state.effect = .submitted key (s.effectFence + 1) := by
  grind [applyCommand]

theorem broker_unknown_success_is_fenced {s : Snapshot} {receipt : BrokerReceipt}
    {applied : Applied}
    (success : applyCommand s (.markEffectUnknown receipt) = .ok applied) :
    BrokerCurrent s receipt := by
  grind [applyCommand]

theorem start_session_requires_admin {s : Snapshot} {admin : AdminToken} {epoch : Nat}
    {applied : Applied}
    (success : applyCommand s (.startOwnerSession admin epoch) = .ok applied) :
    AdminCurrent s admin := by
  grind [applyCommand]

theorem submit_effect_requires_owner_and_environment {s : Snapshot} {token : Token}
    {environmentToken : EnvironmentToken} {key : Nat} {applied : Applied}
    (success : applyCommand s (.submitEffect token environmentToken key) = .ok applied) :
    Authorized s token .operate ∧
    EnvironmentCurrent s environmentToken ∧
    environmentToken.agentId = token.agentId := by
  grind [applyCommand]

theorem current_token_is_orb_owner {s : Snapshot} {token : Token}
    (current : Current s token) : token.agentId = s.ownerSession.agentId := by
  grind [Current]

theorem disconnected_environment_rejects_submit {s : Snapshot} {token : Token}
    {environmentToken : EnvironmentToken} {key : Nat} {applied : Applied}
    (disconnected : s.environment.connected = false) :
    applyCommand s (.submitEffect token environmentToken key) ≠ .ok applied := by
  grind [applyCommand, EnvironmentCurrent]

theorem applyCommand_preserves_permanent_owner {s : Snapshot} {command : Command}
    {applied : Applied} (success : applyCommand s command = .ok applied) :
    applied.state.ownerSession.agentId = s.ownerSession.agentId := by
  cases command <;> grind [applyCommand]

theorem applyCommand_preserves_valid {s : Snapshot} {command : Command}
    {applied : Applied} (valid : Valid s)
    (success : applyCommand s command = .ok applied) : Valid applied.state := by
  cases command <;> grind [applyCommand, Valid, OwnerSafe, Authorized, Current]

theorem valid_ignores_shard_revision (s : Snapshot) (revision : Nat) :
    Valid { s with shardRevision := revision } ↔ Valid s := by
  rfl

theorem step_preserves_valid {s : Snapshot} {envelope : Envelope} {applied : Applied}
    (valid : Valid s) (success : step s envelope = .ok applied) :
    Valid applied.state := by
  grind [step, applyCommand_preserves_valid, valid_ignores_shard_revision]

theorem step_advances_shard_revision {s : Snapshot} {envelope : Envelope}
    {applied : Applied} (success : step s envelope = .ok applied) :
    envelope.expectedShardRevision = s.shardRevision ∧
    applied.state.shardRevision = s.shardRevision + 1 := by
  grind [step]

end AgentOrb
