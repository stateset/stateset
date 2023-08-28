package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgOpenOrder = "open_order"

var _ sdk.Msg = &MsgOpenOrder{}

func NewMsgOpenOrder(creator string, id uint64) *MsgOpenOrder {
  return &MsgOpenOrder{
		Creator: creator,
    Id: id,
	}
}

func (msg *MsgOpenOrder) Route() string {
  return RouterKey
}

func (msg *MsgOpenOrder) Type() string {
  return TypeMsgOpenOrder
}

func (msg *MsgOpenOrder) GetSigners() []sdk.AccAddress {
  creator, err := sdk.AccAddressFromBech32(msg.Creator)
  if err != nil {
    panic(err)
  }
  return []sdk.AccAddress{creator}
}

func (msg *MsgOpenOrder) GetSignBytes() []byte {
  bz := ModuleCdc.MustMarshalJSON(msg)
  return sdk.MustSortJSON(bz)
}

func (msg *MsgOpenOrder) ValidateBasic() error {
  _, err := sdk.AccAddressFromBech32(msg.Creator)
  	if err != nil {
  		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
  	}
  return nil
}

