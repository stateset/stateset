package keeper

import (
	"github.com/stateset/stateset/x/stateset/types"
)

var _ types.QueryServer = Keeper{}
