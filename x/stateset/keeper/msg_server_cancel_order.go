package keeper

import (
	"context"

    "github.com/stateset/stateset/x/stateset/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)


func (k msgServer) CancelOrder(goCtx context.Context,  msg *types.MsgCancelOrder) (*types.MsgCancelOrderResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

    // TODO: Handling the message
    _ = ctx

	return &types.MsgCancelOrderResponse{}, nil
}
