
from lib import showWindow
from PyQt5.QtWidgets import QMessageBox, QApplication, QMainWindow, QPushButton, QVBoxLayout, QWidget, QTabWidget, QCheckBox, QLabel, QLineEdit, QHBoxLayout  

from IPC.message import send_msg

plants_list = [
    "向日葵豌豆射手",
    "火炬坚果",
    "冰豌豆香蒲",
    "普通豌豆香蒲",
]

class plantWindow(QMainWindow):
    def __init__(self, main_window) -> None:
        self.dup_allow = False
        self.penetr_allow = False

        super().__init__()
        self.main_window = main_window

        # 创建植物界面  
        self.tab = QWidget()  
        self.layout = QVBoxLayout()  
        self.label = QLabel("请选择要增加的植物：")  
        self.layout.addWidget(self.label)  
  
        # 创建植物的勾选框  
        self.checkboxes = [QCheckBox(plant) for plant in plants_list]
        for checkbox in self.checkboxes:  
            self.layout.addWidget(checkbox)  
  
        # 创建增加数量和行数的输入框  
        self.count_label = QLabel("增加数量：")  
        self.count_input = QLineEdit()  
        self.row_label = QLabel("添加行数：")  
        self.row_input = QLineEdit()  
        self.col_label = QLabel("添加列数：")  
        self.col_input = QLineEdit()  
  
        # 创建水平布局并添加“增加”控件  
        self.input_layout = QHBoxLayout()  
        self.input_layout.addWidget(self.count_label)  
        self.input_layout.addWidget(self.count_input)  
        self.input_layout.addWidget(self.row_label)  
        self.input_layout.addWidget(self.row_input)  
        self.input_layout.addWidget(self.col_label)
        self.input_layout.addWidget(self.col_input)
        self.layout.addLayout(self.input_layout)  
  
        # 创建提交按钮  
        self.submit_button = QPushButton("提交")  
        self.submit_button.clicked.connect(self.on_submit2)  
        self.layout.addWidget(self.submit_button)  
  
        print("Hello")

        self.label = QLabel("\n\n请选择需要修改的植物属性")  
        self.layout.addWidget(self.label)  
        self.attribute_layout = QHBoxLayout() 

        self.allowDup_btn = QPushButton("允许叠放")
        self.allowDup_btn.clicked.connect(self.on_allowDup_submit) 
        self.attribute_layout.addWidget(self.allowDup_btn)  

        self.allowPenetrate_btn = QPushButton("允许豌豆穿透")
        self.allowPenetrate_btn.clicked.connect(self.on_allowPenetrate_submit)
        self.attribute_layout.addWidget(self.allowPenetrate_btn)

        self.layout.addLayout(self.attribute_layout) 

        self.tab.setLayout(self.layout)  

        self.main_window.addTab(self.tab, "植物修改")

    def on_submit2(self):  
        selected_plants = [checkbox.text() for checkbox in self.checkboxes if checkbox.isChecked()]  

        if len(selected_plants) == 0:  
            QMessageBox.warning(self, "警告", "没有选中任何植物，请至少选择一种植物。")  
            return 

        count = self.count_input.text()  
        row = self.row_input.text()
        col = self.col_input.text()  
        print(f"选中的植物：{selected_plants}")  
        print(f"增加数量：{count}")  
        print(f"添加行数：{row}")  
        print(f"添加列数：{col}")  
  

    def warning_dup_operation(self):
        QMessageBox.warning(self, "警告", "已经设置过该操作")

    def on_allowDup_submit(self):
        if self.dup_allow:
            self.warning_dup_operation()
            return
        send_msg(task="ALLOW_DUPLICATION", data="")
        self.dup_allow = True

    def on_allowPenetrate_submit(self):
        if self.penetr_allow:
            self.warning_dup_operation()
            return
        send_msg(task="ALLOW_PENETRATION", data="")
        self.penetr_allow = True
