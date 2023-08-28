package keeper

import (
	"context"

    "github.com/stateset/stateset/x/stateset/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)


func (k msgServer) CloseOrder(goCtx context.Context,  msg *types.MsgCloseOrder) (*types.MsgCloseOrderResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

    // TODO: Handling the message
    _ = ctx

	return &types.MsgCloseOrderResponse{}, nil
}
