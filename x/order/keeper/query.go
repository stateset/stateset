package keeper

import (
	"github.com/stateset/stateset/x/order/types"
)

var _ types.QueryServer = Keeper{}
