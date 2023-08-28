package keeper

import (
	"context"

    "github.com/stateset/stateset/x/stateset/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)


func (k msgServer) OpenOrder(goCtx context.Context,  msg *types.MsgOpenOrder) (*types.MsgOpenOrderResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

    // TODO: Handling the message
    _ = ctx

	return &types.MsgOpenOrderResponse{}, nil
}
