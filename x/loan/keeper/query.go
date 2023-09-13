package keeper

import (
	"github.com/stateset/stateset/x/loan/types"
)

var _ types.QueryServer = Keeper{}
