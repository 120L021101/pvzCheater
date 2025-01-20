import mmap  
import os  
import threading  
import json
import ctypes  
from ctypes import wintypes  

# 载入Windows API函数  
kernel32 = ctypes.WinDLL('kernel32', use_last_error=True)  
  
# 定义所需的Windows数据结构  
class SECURITY_ATTRIBUTES(ctypes.Structure):  
    _fields_ = [("nLength", wintypes.DWORD),  
                ("lpSecurityDescriptor", ctypes.c_void_p),  
                ("bInheritHandle", wintypes.BOOL)]  
  
# 定义CreateSemaphoreW函数的参数类型和返回类型  
kernel32.CreateSemaphoreW.argtypes = [  
    ctypes.POINTER(SECURITY_ATTRIBUTES),  # lpSemaphoreAttributes  
    wintypes.LONG,                        # lInitialCount  
    wintypes.LONG,                        # lMaximumCount  
    ctypes.c_wchar_p                      # lpName  
]  
kernel32.CreateSemaphoreW.restype = wintypes.HANDLE

# 创建命名信号量  
security_attributes = SECURITY_ATTRIBUTES(0, None, 1)  
initial_count = 0  
maximum_count = 1  
data_ready = kernel32.CreateSemaphoreW(ctypes.byref(security_attributes), initial_count, maximum_count, "Global\\data_ready")
data_received = kernel32.CreateSemaphoreW(ctypes.byref(security_attributes), initial_count, maximum_count, "Global\\data_received")  

if not data_ready or not data_received:  
    raise ctypes.WinError(ctypes.get_last_error())
msg_file = "msg_shared_memory"
with open(msg_file, "wb") as f:
    f.seek(1024 - 1)
    f.write(b"\x00")  


def send_msg(task, data):
    msg = json.dumps({"TASK" : task, "DATA" : json.dumps(data, ensure_ascii=False)}, ensure_ascii=False).encode()
    msg_len = len(msg).to_bytes(4)
    global msg_file

    with open(msg_file, "r+b") as f:  
        mm = mmap.mmap(f.fileno(), length=0, access=mmap.ACCESS_WRITE)  
        # 写入数据并通知Rust  
        mm.write(msg_len)
        mm.write(msg)  
        mm.flush()  
        mm.close()
    previous_count = wintypes.LONG()
    if not kernel32.ReleaseSemaphore(data_ready, 1, ctypes.byref(previous_count)):  
        raise ctypes.WinError(ctypes.get_last_error())  

    # 等待Rust读取数据  
    WAIT_INFINITE = 0xFFFFFFFF  
    print("Rust has read the data.")  
    