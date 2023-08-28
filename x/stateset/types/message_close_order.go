package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgCloseOrder = "close_order"

var _ sdk.Msg = &MsgCloseOrder{}

func NewMsgCloseOrder(creator string, id uint64) *MsgCloseOrder {
  return &MsgCloseOrder{
		Creator: creator,
    Id: id,
	}
}

func (msg *MsgCloseOrder) Route() string {
  return RouterKey
}

func (msg *MsgCloseOrder) Type() string {
  return TypeMsgCloseOrder
}

func (msg *MsgCloseOrder) GetSigners() []sdk.AccAddress {
  creator, err := sdk.AccAddressFromBech32(msg.Creator)
  if err != nil {
    panic(err)
  }
  return []sdk.AccAddress{creator}
}

func (msg *MsgCloseOrder) GetSignBytes() []byte {
  bz := ModuleCdc.MustMarshalJSON(msg)
  return sdk.MustSortJSON(bz)
}

func (msg *MsgCloseOrder) ValidateBasic() error {
  _, err := sdk.AccAddressFromBech32(msg.Creator)
  	if err != nil {
  		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
  	}
  return nil
}

