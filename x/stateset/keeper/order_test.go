package keeper_test

import (
	"testing"

    "github.com/stateset/stateset/x/stateset/keeper"
    "github.com/stateset/stateset/x/stateset/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	keepertest "github.com/stateset/stateset/testutil/keeper"
	"github.com/stateset/stateset/testutil/nullify"
	"github.com/stretchr/testify/require"
)

func createNOrder(keeper *keeper.Keeper, ctx sdk.Context, n int) []types.Order {
	items := make([]types.Order, n)
	for i := range items {
		items[i].Id = keeper.AppendOrder(ctx, items[i])
	}
	return items
}

func TestOrderGet(t *testing.T) {
	keeper, ctx := keepertest.StatesetKeeper(t)
	items := createNOrder(keeper, ctx, 10)
	for _, item := range items {
		got, found := keeper.GetOrder(ctx, item.Id)
		require.True(t, found)
		require.Equal(t,
			nullify.Fill(&item),
			nullify.Fill(&got),
		)
	}
}

func TestOrderRemove(t *testing.T) {
	keeper, ctx := keepertest.StatesetKeeper(t)
	items := createNOrder(keeper, ctx, 10)
	for _, item := range items {
		keeper.RemoveOrder(ctx, item.Id)
		_, found := keeper.GetOrder(ctx, item.Id)
		require.False(t, found)
	}
}

func TestOrderGetAll(t *testing.T) {
	keeper, ctx := keepertest.StatesetKeeper(t)
	items := createNOrder(keeper, ctx, 10)
	require.ElementsMatch(t,
		nullify.Fill(items),
		nullify.Fill(keeper.GetAllOrder(ctx)),
	)
}

func TestOrderCount(t *testing.T) {
	keeper, ctx := keepertest.StatesetKeeper(t)
	items := createNOrder(keeper, ctx, 10)
	count := uint64(len(items))
	require.Equal(t, count, keeper.GetOrderCount(ctx))
}
