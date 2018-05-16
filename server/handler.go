package server

type Handler struct {
	TLog TLog;
}


func (h *Handler) Replay(key string, start string, end string)([]byte, error) {
	return []byte("BEAM/" + string("")), nil
}


func (h *Handler) SET(key string, value []byte) error {
	ns := h.TLog.GetOrCreateNameSpace(key)
	return ns.write(value)
}
