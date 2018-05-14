import json
import time
from queue import Queue


class Connection(object):
    def flush(self):
        pass

class LocalData():
    def __init__(self, data):
        self.data = data


class Data(object):
    def __init__(self, config, queue):
        self.config = config
        self.current = b" " * config['global']['max_size_before_flush']
        self.spare = None
        self.spare_allocated = False
        self.curr_idx = -1
        self.queue = queue

    def write(self, data):
        if  not self.spare_allocated:
            self.spare = b" " * self.config['global']['max_size_before_flush']
            self.spare_allocated = True

        if self.curr_idx == len(self.current) - 1:
            self.queue.put(LocalData(self.current))
            self.current = self.spare
            self.spare = b" " * self.config['global']['max_size_before_flush']
            self.curr_idx = -1

        now = str(int(time.time())).encode()

        data = now + data
        for e in data:
            self.curr_idx += 1
            self.current[self.curr_idx] = e


class MetaData(object):
    def __init__(self, config):
        self._md = {0: b" " * self.config['global']['max_size_before_flush']}
        self.config = config
        self.curr_dict_idx = 0
        self.curr_idx = -1
        self.starting_idx = 0
        self.used = 0
        self._iter_position = (0, 0)
        self._iter_end = (0, 0)

    def _append(self, data):
        length = len(data)
        if self.used == self.config['global']['max_size_before_flush']:
            self.curr_dict_idx +=1
            self.starting_idx = 0
            self.used = 0
            self._md[self.curr_dict_idx] = b" " * self.config['global']['max_size_before_flush']

        for e in data:
            self.curr_idx += 1
            self._md[self.curr_dict_idx][self.curr_idx] = e

    def search(self, timestamp):
        """
        Find
        :return:
        """

    def __iter__ (self):
        return self

    def set_replay(self, start=None, end=None):
        """
        Set iterator starting and end points
        """

        if start is None:
            self._iter_position = (self.starting_idx, 0)
        else:
            self._iter_position = self.search(start)


        if end is None:
            self._iter_end = (self.curr_idx, 0)
        else:
            self._iter_end = self.search(end)

    def __next__(self):
        curr_dict_idx, curr_buff_idx = self._iter_position
        end_dict_idx, end_buff_idx = self._iter_end

        if curr_dict_idx > end_dict_idx:
            raise StopIteration

        curr_buf = self._md[curr_dict_idx]

        if curr_buff_idx == len(curr_buf) - 1:
            curr_dict_idx += 1

        blk_size = self.self.config['global']['block_size']
        data =  curr_buf[curr_buff_idx: curr_buff_idx + blk_size]
        curr_buff_idx += blk_size
        return data


    def __iter__(self):
        return self


class Namespace(object):
    def __init__(self, config, connection):
        self.config = config
        self.connection = connection
        self.queue = Queue()
        self.data = Data(self.config, self.queue)
        self.metadata = MetaData(self.config)
        self._create_flusher(self.queue, self.data, self.metadata, self.connection)
        self._create_md_cleander(self.metadata, self.connection)

    def _create_flusher(self, queue, data, metadata, connection):
        """
        Async task to flush data into backend and put metadata into metadata
        """

    def _create_md_cleander(self, md, connection):
        """
        Async task to clean md older than certain date
        until first valid meta data, then waits
        """


class Tlog(object):

    def __init__(self, config):
        self.config = config
        self.namespaces = {}
        self.connection = Connection()

    def create_namespace(self, name, config):
        conf = {'global': self.config, 'local': config}
        self.namespaces['name'] = Namespace(conf, self.connection)

if __name__ == '__main__':
    conf = json.load(open('settings.json'))
    tlog = Tlog(conf)
    tlog.create_namespace('ns1', {})
