package keeper

import (
	"context"

    "github.com/stateset/stateset/x/stateset/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)


func (k msgServer) CompleteOrder(goCtx context.Context,  msg *types.MsgCompleteOrder) (*types.MsgCompleteOrderResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

    // TODO: Handling the message
    _ = ctx

	return &types.MsgCompleteOrderResponse{}, nil
}
