package stateset

import (
	"math/rand"

	"github.com/cosmos/cosmos-sdk/baseapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
	"github.com/cosmos/cosmos-sdk/x/simulation"
	"github.com/stateset/stateset/testutil/sample"
	statesetsimulation "github.com/stateset/stateset/x/stateset/simulation"
	"github.com/stateset/stateset/x/stateset/types"
)

// avoid unused import issue
var (
	_ = sample.AccAddress
	_ = statesetsimulation.FindAccount
	_ = simulation.MsgEntryKind
	_ = baseapp.Paramspace
	_ = rand.Rand{}
)

const (
opWeightMsgCompleteOrder = "op_weight_msg_complete_order"
	// TODO: Determine the simulation weight value
	defaultWeightMsgCompleteOrder int = 100

	opWeightMsgCancelOrder = "op_weight_msg_cancel_order"
	// TODO: Determine the simulation weight value
	defaultWeightMsgCancelOrder int = 100

	opWeightMsgCloseOrder = "op_weight_msg_close_order"
	// TODO: Determine the simulation weight value
	defaultWeightMsgCloseOrder int = 100

	opWeightMsgOpenOrder = "op_weight_msg_open_order"
	// TODO: Determine the simulation weight value
	defaultWeightMsgOpenOrder int = 100

	// this line is used by starport scaffolding # simapp/module/const
)

// GenerateGenesisState creates a randomized GenState of the module.
func (AppModule) GenerateGenesisState(simState *module.SimulationState) {
	accs := make([]string, len(simState.Accounts))
	for i, acc := range simState.Accounts {
		accs[i] = acc.Address.String()
	}
	statesetGenesis := types.GenesisState{
		Params: types.DefaultParams(),
		// this line is used by starport scaffolding # simapp/module/genesisState
	}
	simState.GenState[types.ModuleName] = simState.Cdc.MustMarshalJSON(&statesetGenesis)
}

// RegisterStoreDecoder registers a decoder.
func (am AppModule) RegisterStoreDecoder(_ sdk.StoreDecoderRegistry) {}

// ProposalContents doesn't return any content functions for governance proposals.
func (AppModule) ProposalContents(_ module.SimulationState) []simtypes.WeightedProposalContent {
	return nil
}

// WeightedOperations returns the all the gov module operations with their respective weights.
func (am AppModule) WeightedOperations(simState module.SimulationState) []simtypes.WeightedOperation {
	operations := make([]simtypes.WeightedOperation, 0)

	var weightMsgCompleteOrder int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgCompleteOrder, &weightMsgCompleteOrder, nil,
		func(_ *rand.Rand) {
			weightMsgCompleteOrder = defaultWeightMsgCompleteOrder
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgCompleteOrder,
		statesetsimulation.SimulateMsgCompleteOrder(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgCancelOrder int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgCancelOrder, &weightMsgCancelOrder, nil,
		func(_ *rand.Rand) {
			weightMsgCancelOrder = defaultWeightMsgCancelOrder
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgCancelOrder,
		statesetsimulation.SimulateMsgCancelOrder(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgCloseOrder int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgCloseOrder, &weightMsgCloseOrder, nil,
		func(_ *rand.Rand) {
			weightMsgCloseOrder = defaultWeightMsgCloseOrder
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgCloseOrder,
		statesetsimulation.SimulateMsgCloseOrder(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	var weightMsgOpenOrder int
	simState.AppParams.GetOrGenerate(simState.Cdc, opWeightMsgOpenOrder, &weightMsgOpenOrder, nil,
		func(_ *rand.Rand) {
			weightMsgOpenOrder = defaultWeightMsgOpenOrder
		},
	)
	operations = append(operations, simulation.NewWeightedOperation(
		weightMsgOpenOrder,
		statesetsimulation.SimulateMsgOpenOrder(am.accountKeeper, am.bankKeeper, am.keeper),
	))

	// this line is used by starport scaffolding # simapp/module/operation

	return operations
}

// ProposalMsgs returns msgs used for governance proposals for simulations.
func (am AppModule) ProposalMsgs(simState module.SimulationState) []simtypes.WeightedProposalMsg {
	return []simtypes.WeightedProposalMsg{
		simulation.NewWeightedProposalMsg(
	opWeightMsgCompleteOrder,
	defaultWeightMsgCompleteOrder,
	func(r *rand.Rand, ctx sdk.Context, accs []simtypes.Account) sdk.Msg {
		statesetsimulation.SimulateMsgCompleteOrder(am.accountKeeper, am.bankKeeper, am.keeper)
		return nil
	},
),
simulation.NewWeightedProposalMsg(
	opWeightMsgCancelOrder,
	defaultWeightMsgCancelOrder,
	func(r *rand.Rand, ctx sdk.Context, accs []simtypes.Account) sdk.Msg {
		statesetsimulation.SimulateMsgCancelOrder(am.accountKeeper, am.bankKeeper, am.keeper)
		return nil
	},
),
simulation.NewWeightedProposalMsg(
	opWeightMsgCloseOrder,
	defaultWeightMsgCloseOrder,
	func(r *rand.Rand, ctx sdk.Context, accs []simtypes.Account) sdk.Msg {
		statesetsimulation.SimulateMsgCloseOrder(am.accountKeeper, am.bankKeeper, am.keeper)
		return nil
	},
),
simulation.NewWeightedProposalMsg(
	opWeightMsgOpenOrder,
	defaultWeightMsgOpenOrder,
	func(r *rand.Rand, ctx sdk.Context, accs []simtypes.Account) sdk.Msg {
		statesetsimulation.SimulateMsgOpenOrder(am.accountKeeper, am.bankKeeper, am.keeper)
		return nil
	},
),
// this line is used by starport scaffolding # simapp/module/OpMsg
	}
}
