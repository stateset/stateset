package order_test

import (
	"testing"

	keepertest "github.com/stateset/stateset/testutil/keeper"
	"github.com/stateset/stateset/testutil/nullify"
	"github.com/stateset/stateset/x/order"
	"github.com/stateset/stateset/x/order/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	genesisState := types.GenesisState{
		Params:	types.DefaultParams(),
		
		// this line is used by starport scaffolding # genesis/test/state
	}

	k, ctx := keepertest.OrderKeeper(t)
	order.InitGenesis(ctx, *k, genesisState)
	got := order.ExportGenesis(ctx, *k)
	require.NotNil(t, got)

	nullify.Fill(&genesisState)
	nullify.Fill(got)

	

	// this line is used by starport scaffolding # genesis/test/assert
}
