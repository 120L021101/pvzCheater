
from lib import showWindow
from PyQt5.QtWidgets import QMessageBox, QApplication, QMainWindow, QPushButton, QVBoxLayout, QWidget, QTabWidget, QCheckBox, QLabel, QLineEdit, QHBoxLayout  

from IPC.message import send_msg

class miscWindow(QMainWindow):
    def __init__(self, main_window) -> None:
        super().__init__()
        self.main_window = main_window

        self.tab = QWidget()
        self.layout = QVBoxLayout()
        self.tab.setLayout(self.layout)


        # 修改阳光的功能
        self.sun_num_label = QLabel("修改阳光的数量：")  
        self.sun_num_input = QLineEdit()  
        self.input_layout = QHBoxLayout()  
        self.input_layout.addWidget(self.sun_num_label)  
        self.input_layout.addWidget(self.sun_num_input)  
        self.layout.addLayout(self.input_layout)  
        self.submit_button = QPushButton("提交")  
        self.submit_button.clicked.connect(self.modify_sun_commit)  
        self.layout.addWidget(self.submit_button)  


        self.main_window.addTab(self.tab, "杂项修改")

    def modify_sun_commit(self) -> None:

        count = self.sun_num_input.text()  
        print(f"修改数量：{count}")  
        send_msg(task="MODIFY_SUN_VALUE", data={"modify_sun_value" : int(count)})
  