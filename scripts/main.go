package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"flag"
	"fmt"
	"math/big"
	"os"
	"path/filepath"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
)

type TxFixtureFile struct {
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

type ReceiptFixtureFile struct {
	Vectors []ReceiptFixture `json:"vectors"`
}

type ReceiptFixture struct {
	Name   string                 `json:"name"`
	TxType string                 `json:"receipt_type"`
	Raw    string                 `json:"raw"`
	Expect ReceiptAccessorFixture `json:"expect"`
}

type ReceiptAccessorFixture struct {
	Status            bool   `json:"status"`
	CumulativeGasUsed uint64 `json:"cumulative_gas_used"`
	GasUsedForL1      uint64 `json:"gas_used_for_l1"`
	LogsLen           int    `json:"logs_len"`
}

type HeaderFixtureFile struct {
	Vectors []HeaderFixture `json:"vectors"`
}

type HeaderFixture struct {
	Name      string       `json:"name"`
	ExtraData string       `json:"extra_data"`
	MixHash   string       `json:"mix_hash"`
	Expect    HeaderExpect `json:"expect"`
}

type HeaderExpect struct {
	SendRoot           string `json:"send_root"`
	SendCount          uint64 `json:"send_count"`
	L1BlockNumber      uint64 `json:"l1_block_number"`
	ArbOSFormatVersion uint64 `json:"arbos_format_version"`
}

func main() {
	outPath := flag.String("out", "", "legacy alias for -tx-out")
	txOutPath := flag.String("tx-out", "", "output path for tx fixtures json")
	receiptOutPath := flag.String("receipt-out", "", "output path for receipt fixtures json")
	headerOutPath := flag.String("header-out", "", "output path for header fixtures json")
	flag.Parse()

	if *txOutPath == "" {
		*txOutPath = *outPath
	}

	if *txOutPath == "" {
		fmt.Fprintln(os.Stderr, "missing required -tx-out path (or legacy -out)")
		os.Exit(2)
	}

	txFixtures := generateTxFixtures()
	if err := writeJSON(*txOutPath, TxFixtureFile{Vectors: txFixtures}); err != nil {
		panic(err)
	}

	if *receiptOutPath != "" {
		receiptFixtures := generateReceiptFixtures()
		if err := writeJSON(*receiptOutPath, ReceiptFixtureFile{Vectors: receiptFixtures}); err != nil {
			panic(err)
		}
	}

	if *headerOutPath != "" {
		headerFixtures := generateHeaderFixtures()
		if err := writeJSON(*headerOutPath, HeaderFixtureFile{Vectors: headerFixtures}); err != nil {
			panic(err)
		}
	}
}

func generateTxFixtures() []TxFixture {
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

	return []TxFixture{
		mustFixture("unsigned_call", types.NewTx(unsigned), addr1),
		mustFixture("contract_call", types.NewTx(contract), addr1),
		mustFixture("retry_call", types.NewTx(retry), addr1),
		mustFixture("submit_retryable", types.NewTx(submit), addr1),
		mustFixture("deposit", types.NewTx(deposit), addr1),
		mustFixture("internal", types.NewTx(internal), types.ArbosAddress),
	}
}

func generateReceiptFixtures() []ReceiptFixture {
	addr1 := common.HexToAddress("0x1111111111111111111111111111111111111111")
	topic1 := common.HexToHash("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")

	withLog := []*types.Log{{
		Address: addr1,
		Topics:  []common.Hash{topic1},
		Data:    []byte{0xde, 0xad, 0xbe, 0xef},
	}}

	return []ReceiptFixture{
		mustReceiptFixture("legacy_success", types.LegacyTxType, types.ReceiptStatusSuccessful, 21_000, nil),
		mustReceiptFixture("eip1559_failed", types.DynamicFeeTxType, types.ReceiptStatusFailed, 42_000, withLog),
		mustReceiptFixture("deposit_success", types.ArbitrumDepositTxType, types.ReceiptStatusSuccessful, 63_000, nil),
		mustReceiptFixture("unsigned_success", types.ArbitrumUnsignedTxType, types.ReceiptStatusSuccessful, 84_000, nil),
		mustReceiptFixture("contract_failed", types.ArbitrumContractTxType, types.ReceiptStatusFailed, 105_000, nil),
		mustReceiptFixture("retry_success", types.ArbitrumRetryTxType, types.ReceiptStatusSuccessful, 126_000, withLog),
		mustReceiptFixture("submit_retryable_success", types.ArbitrumSubmitRetryableTxType, types.ReceiptStatusSuccessful, 147_000, nil),
		mustReceiptFixture("internal_success", types.ArbitrumInternalTxType, types.ReceiptStatusSuccessful, 168_000, nil),
	}
}

func generateHeaderFixtures() []HeaderFixture {
	return []HeaderFixture{
		mustHeaderFixture("header_basic", types.HeaderInfo{
			SendRoot:           common.HexToHash("0x1111111111111111111111111111111111111111111111111111111111111111"),
			SendCount:          42,
			L1BlockNumber:      99_001,
			ArbOSFormatVersion: 32,
		}),
		mustHeaderFixture("header_non_arbitrum", types.HeaderInfo{
			SendRoot:           common.HexToHash("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
			SendCount:          1,
			L1BlockNumber:      8_888_888,
			ArbOSFormatVersion: 0,
		}),
		mustHeaderFixture("header_large_values", types.HeaderInfo{
			SendRoot:           common.HexToHash("0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
			SendCount:          ^uint64(0),
			L1BlockNumber:      ^uint64(0) - 1,
			ArbOSFormatVersion: 50,
		}),
	}
}

func mustReceiptFixture(name string, receiptType uint8, status uint64, cumulativeGasUsed uint64, logs []*types.Log) ReceiptFixture {
	receipt := &types.Receipt{
		Type:              receiptType,
		Status:            status,
		CumulativeGasUsed: cumulativeGasUsed,
		GasUsedForL1:      0,
		Logs:              logs,
	}
	receipt.Bloom = types.CreateBloom(receipt)

	raw, err := receipt.MarshalBinary()
	if err != nil {
		panic(err)
	}

	return ReceiptFixture{
		Name:   name,
		TxType: fmt.Sprintf("0x%02x", receiptType),
		Raw:    "0x" + hex.EncodeToString(raw),
		Expect: ReceiptAccessorFixture{
			Status:            status == types.ReceiptStatusSuccessful,
			CumulativeGasUsed: cumulativeGasUsed,
			GasUsedForL1:      receipt.GasUsedForL1,
			LogsLen:           len(logs),
		},
	}
}

func mustHeaderFixture(name string, info types.HeaderInfo) HeaderFixture {
	header := &types.Header{
		BaseFee:    big.NewInt(1),
		Difficulty: common.Big1,
	}
	info.UpdateHeaderWithInfo(header)

	decoded := types.DeserializeHeaderExtraInformation(header)
	if decoded != info {
		panic(fmt.Sprintf("header info roundtrip mismatch for %s", name))
	}

	return HeaderFixture{
		Name:      name,
		ExtraData: "0x" + hex.EncodeToString(header.Extra),
		MixHash:   header.MixDigest.Hex(),
		Expect: HeaderExpect{
			SendRoot:           info.SendRoot.Hex(),
			SendCount:          info.SendCount,
			L1BlockNumber:      info.L1BlockNumber,
			ArbOSFormatVersion: info.ArbOSFormatVersion,
		},
	}
}

func writeJSON(path string, value interface{}) error {
	data, err := json.MarshalIndent(value, "", "  ")
	if err != nil {
		return err
	}
	if !bytes.HasSuffix(data, []byte{'\n'}) {
		data = append(data, '\n')
	}

	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	return os.WriteFile(path, data, 0o644)
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
