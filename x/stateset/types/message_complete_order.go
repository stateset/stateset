package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgCompleteOrder = "complete_order"

var _ sdk.Msg = &MsgCompleteOrder{}

func NewMsgCompleteOrder(creator string, id uint64) *MsgCompleteOrder {
  return &MsgCompleteOrder{
		Creator: creator,
    Id: id,
	}
}

func (msg *MsgCompleteOrder) Route() string {
  return RouterKey
}

func (msg *MsgCompleteOrder) Type() string {
  return TypeMsgCompleteOrder
}

func (msg *MsgCompleteOrder) GetSigners() []sdk.AccAddress {
  creator, err := sdk.AccAddressFromBech32(msg.Creator)
  if err != nil {
    panic(err)
  }
  return []sdk.AccAddress{creator}
}

func (msg *MsgCompleteOrder) GetSignBytes() []byte {
  bz := ModuleCdc.MustMarshalJSON(msg)
  return sdk.MustSortJSON(bz)
}

func (msg *MsgCompleteOrder) ValidateBasic() error {
  _, err := sdk.AccAddressFromBech32(msg.Creator)
  	if err != nil {
  		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
  	}
  return nil
}

