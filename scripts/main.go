package main

import (
	"encoding/hex"
	"encoding/json"
	"flag"
	"fmt"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
)

type FixtureFile struct {
	Vectors []TxFixture `json:"vectors"`
}

type TxFixture struct {
	Name   string          `json:"name"`
	TxType string          `json:"tx_type"`
	Raw    string          `json:"raw"`
	Hash   string          `json:"hash"`
	Expect AccessorFixture `json:"expect"`
}

type AccessorFixture struct {
	From     string  `json:"from"`
	To       *string `json:"to"`
	Nonce    uint64  `json:"nonce"`
	GasLimit uint64  `json:"gas_limit"`
	Value    string  `json:"value"`
	Input    string  `json:"input"`
}

func main() {
	outPath := flag.String("out", "", "output path for fixture json")
	flag.Parse()

	if *outPath == "" {
		fmt.Fprintln(os.Stderr, "missing required -out path")
		os.Exit(2)
	}

	chainID := big.NewInt(42161)

	addr1 := common.HexToAddress("0x1111111111111111111111111111111111111111")
	addr2 := common.HexToAddress("0x2222222222222222222222222222222222222222")
	addr3 := common.HexToAddress("0x3333333333333333333333333333333333333333")
	addr4 := common.HexToAddress("0x4444444444444444444444444444444444444444")
	addr5 := common.HexToAddress("0x5555555555555555555555555555555555555555")

	req1 := common.HexToHash("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
	req2 := common.HexToHash("0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
	req3 := common.HexToHash("0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc")

	unsigned := &types.ArbitrumUnsignedTx{
		ChainId:   chainID,
		From:      addr1,
		Nonce:     7,
		GasFeeCap: big.NewInt(12345),
		Gas:       21000,
		To:        &addr2,
		Value:     big.NewInt(100),
		Data:      []byte{0xde, 0xad, 0xbe, 0xef},
	}

	contract := &types.ArbitrumContractTx{
		ChainId:   chainID,
		RequestId: req1,
		From:      addr1,
		GasFeeCap: big.NewInt(12345),
		Gas:       50000,
		To:        &addr2,
		Value:     big.NewInt(1),
		Data:      []byte{0xab, 0xcd, 0xef},
	}

	retry := &types.ArbitrumRetryTx{
		ChainId:             chainID,
		Nonce:               3,
		From:                addr1,
		GasFeeCap:           big.NewInt(555),
		Gas:                 80000,
		To:                  &addr2,
		Value:               big.NewInt(5),
		Data:                []byte{0x01, 0x02},
		TicketId:            req2,
		RefundTo:            addr3,
		MaxRefund:           big.NewInt(1000),
		SubmissionFeeRefund: big.NewInt(2000),
	}

	submit := &types.ArbitrumSubmitRetryableTx{
		ChainId:          chainID,
		RequestId:        req1,
		From:             addr1,
		L1BaseFee:        big.NewInt(100),
		DepositValue:     big.NewInt(200),
		GasFeeCap:        big.NewInt(300),
		Gas:              400000,
		RetryTo:          &addr2,
		RetryValue:       big.NewInt(50),
		Beneficiary:      addr4,
		MaxSubmissionFee: big.NewInt(600),
		FeeRefundAddr:    addr5,
		RetryData:        []byte{0x01, 0x02, 0x03, 0x04},
	}

	deposit := &types.ArbitrumDepositTx{
		ChainId:     chainID,
		L1RequestId: req3,
		From:        addr1,
		To:          addr2,
		Value:       big.NewInt(123),
	}

	internal := &types.ArbitrumInternalTx{
		ChainId: chainID,
		Data:    []byte{0x11, 0x22, 0x33, 0x44, 0x55},
	}

	vectors := []TxFixture{
		mustFixture("unsigned_call", types.NewTx(unsigned), addr1),
		mustFixture("contract_call", types.NewTx(contract), addr1),
		mustFixture("retry_call", types.NewTx(retry), addr1),
		mustFixture("submit_retryable", types.NewTx(submit), addr1),
		mustFixture("deposit", types.NewTx(deposit), addr1),
		mustFixture("internal", types.NewTx(internal), types.ArbosAddress),
	}

	data, err := json.MarshalIndent(FixtureFile{Vectors: vectors}, "", "  ")
	if err != nil {
		panic(err)
	}
	data = append(data, '\n')

	if err := os.WriteFile(*outPath, data, 0o644); err != nil {
		panic(err)
	}
}

func mustFixture(name string, tx *types.Transaction, from common.Address) TxFixture {
	raw, err := tx.MarshalBinary()
	if err != nil {
		panic(err)
	}

	to := tx.To()
	var toHex *string
	if to != nil {
		s := to.Hex()
		toHex = &s
	}

	return TxFixture{
		Name:   name,
		TxType: fmt.Sprintf("0x%02x", tx.Type()),
		Raw:    "0x" + hex.EncodeToString(raw),
		Hash:   tx.Hash().Hex(),
		Expect: AccessorFixture{
			From:     from.Hex(),
			To:       toHex,
			Nonce:    tx.Nonce(),
			GasLimit: tx.Gas(),
			Value:    fmt.Sprintf("0x%x", tx.Value()),
			Input:    "0x" + hex.EncodeToString(tx.Data()),
		},
	}
}
