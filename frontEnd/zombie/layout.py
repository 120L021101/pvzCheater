
from lib import showWindow
from PyQt5.QtWidgets import QMessageBox, QApplication, QMainWindow, QPushButton, QVBoxLayout, QWidget, QTabWidget, QCheckBox, QLabel, QLineEdit, QHBoxLayout  

from IPC.message import send_msg

zombies_list = [
    "普通僵尸",
    "路障僵尸",
    "铁桶僵尸",
    "撑杆跳僵尸",
]

class zombieWindow(QMainWindow):
    def __init__(self, main_window) -> None:
        super().__init__()
        self.main_window = main_window

        # 创建僵尸界面  
        self.tab = QWidget()  
        self.layout = QVBoxLayout()  
        self.label = QLabel("请选择要增加的僵尸：")  
        self.layout.addWidget(self.label)  
  
        # 创建僵尸的勾选框  
        self.checkboxes = [QCheckBox(zombie) for zombie in zombies_list]
        # self.checkboxes = [QCheckBox(f"僵尸{i+1}") for i in range(10)]  
        for checkbox in self.checkboxes:  
            self.layout.addWidget(checkbox)  
  
        # 创建增加数量和行数的输入框  
        self.count_label = QLabel("增加数量：")  
        self.count_input = QLineEdit()  
        self.row_label = QLabel("添加行数：")  
        self.row_input = QLineEdit()  
  
        # 创建水平布局并添加“增加”控件  
        self.input_layout = QHBoxLayout()  
        self.input_layout.addWidget(self.count_label)  
        self.input_layout.addWidget(self.count_input)  
        self.input_layout.addWidget(self.row_label)  
        self.input_layout.addWidget(self.row_input)  
        self.layout.addLayout(self.input_layout)  

          
        # 将三个tab添加到tab_widget中  
        self.main_window.addTab(self.tab, "僵尸界面")  
  
        # 创建提交按钮  
        self.submit_button = QPushButton("提交")  
        self.submit_button.clicked.connect(self.on_submit)  
        self.layout.addWidget(self.submit_button)  
  

        self.label = QLabel("\n\n请选择需要修改的僵尸属性")  
        self.layout.addWidget(self.label)  
        # 创建水平布局并添加“魅惑”控件，“去甲”控件
        self.attribute_layout = QHBoxLayout() 

        self.attr_btn = QPushButton("魅惑僵尸")
        self.attr_btn.clicked.connect(self.on_attr_submit) 
        self.attribute_layout.addWidget(self.attr_btn)  

        self.dedef1_btn = QPushButton("僵尸去护甲1")
        self.dedef1_btn.clicked.connect(self.on_dedef1_submit) 
        self.attribute_layout.addWidget(self.dedef1_btn) 

        self.dedef2_btn = QPushButton("僵尸去护甲2")
        self.dedef2_btn.clicked.connect(self.on_dedef2_submit) 
        self.attribute_layout.addWidget(self.dedef2_btn) 

        self.layout.addLayout(self.attribute_layout) 

        self.tab.setLayout(self.layout)  

        self.main_window.addTab(self.tab, "僵尸修改")

    def on_submit(self):  
        selected_plants = [checkbox.text() for checkbox in self.checkboxes if checkbox.isChecked()]  

        if len(selected_plants) == 0:  
            QMessageBox.warning(self, "警告", "没有选中任何僵尸，请至少选择一种僵尸。")  
            return 

        count = self.count_input.text()  
        row = self.row_input.text()  
        print(f"选中的僵尸：{selected_plants}")  
        print(f"增加数量：{count}")  
        print(f"添加行数：{row}")  
  
    
    def on_attr_submit(self):
        send_msg(task="ATTRACT_ZOMBIE", data="")

    def on_dedef1_submit(self):
        send_msg(task="REMOVE_ARMOR1", data="")

    def on_dedef2_submit(self):
        send_msg(task="REMOVE_ARMOR2", data="")