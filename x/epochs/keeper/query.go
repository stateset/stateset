package keeper

import (
	"github.com/stateset/stateset/x/epochs/types"
)

var _ types.QueryServer = Keeper{}
