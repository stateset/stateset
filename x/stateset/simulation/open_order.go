package simulation

import (
	"math/rand"

	"github.com/stateset/stateset/x/stateset/keeper"
	"github.com/stateset/stateset/x/stateset/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	sdk "github.com/cosmos/cosmos-sdk/types"
	simtypes "github.com/cosmos/cosmos-sdk/types/simulation"
)

func SimulateMsgOpenOrder(
	ak types.AccountKeeper,
	bk types.BankKeeper,
	k keeper.Keeper,
) simtypes.Operation {
	return func(r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
	) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
		simAccount, _ := simtypes.RandomAcc(r, accs)
		msg := &types.MsgOpenOrder{
			Creator: simAccount.Address.String(),
		}

		// TODO: Handling the OpenOrder simulation

		return simtypes.NoOpMsg(types.ModuleName, msg.Type(), "OpenOrder simulation not implemented"), nil, nil
	}
}
