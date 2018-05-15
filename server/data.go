package server

import (
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"time"
	"tlog/config"
)

type Data struct {
	Config      *config.Config
	mainBuffer  bytes.Buffer
	flushBuffer bytes.Buffer
}

func (d *Data) write(data []byte) error {

	maxBuf := int(d.Config.MaxSizeB4Flush)
	if len(data)+10 > maxBuf {
		return errors.New("Data length cannot be greater than max length before flush -10")
	}

	timeStamp := make([]byte, 8)
	binary.LittleEndian.PutUint64(timeStamp, uint64(time.Now().UnixNano()))

	dataLength := make([]byte, 2)
	binary.LittleEndian.PutUint16(timeStamp, uint16(len(data)))

	if len(data)+d.mainBuffer.Len()+10 > maxBuf { // 10 is 8 bytes for time stamp + 2 bytes for data length
		d.flushBuffer = d.mainBuffer
		d.mainBuffer.Reset()
		d.flush()
	}

	d.mainBuffer.Write(timeStamp)
	d.mainBuffer.Write(dataLength)
	d.mainBuffer.Write(data)
	return nil
}

func (d *Data) flush() {
	fmt.Println("Flushing: ", d.flushBuffer.Bytes())
}

func NewData(config *config.Config) *Data {
	return &Data{Config: config}
}
