import sys  
from PyQt5.QtWidgets import QComboBox, QPushButton, QAction, \
                            QDialog, QApplication, QMainWindow, \
                            QMenuBar, QVBoxLayout, QWidget, \
                            QTabWidget, QCheckBox, QLabel, \
                            QLineEdit, QHBoxLayout, QListWidget, \
                            QMessageBox
import psutil
import re

from IPC.message import send_msg

class MenuWindow(QMainWindow):
    def __init__(self, main_window) -> None:
        super().__init__()

        self.main_window = main_window

        self.openMenu = self.main_window.addMenu("打开")  
        self.helpMenu = self.main_window.addMenu("帮助")  
        # 添加菜单项和动作  
        self.openAction = QAction("绑定pvz程序", self)  
        self.openAction.triggered.connect(self.openOpenDialog)  
        self.openMenu.addAction(self.openAction) 
  
        self.aboutAction = QAction("关于", self)  
        self.aboutAction.triggered.connect(self.openAboutDialog)  
        self.helpMenu.addAction(self.aboutAction)  

    def openOpenDialog(self):  
        dialog = QDialog(self)  
        dialog.setWindowTitle("绑定pvz程序，请找到pvz进程")  
        dialog.setGeometry(200, 200, 800, 600)  
        layout = QVBoxLayout()  
        label = QLabel("请找到你想修改的植物大战僵尸进程")  
        layout.addWidget(label)  

        self.listWidget = QListWidget(self)  
        self.fillList()  
        layout.addWidget(self.listWidget)  


        self.input_layout = QHBoxLayout()  
        self.refreshButton = QPushButton("刷新列表")  
        self.refreshButton.clicked.connect(self.onRefreshButtonClick)  
        self.input_layout.addWidget(self.refreshButton)  
        self.commitButton = QPushButton("提交")  
        self.commitButton.clicked.connect(self.onCommitButtonClick)  
        self.input_layout.addWidget(self.commitButton)  
        layout.addLayout(self.input_layout)
  
        centralWidget = QWidget()  
        centralWidget.setLayout(layout)  
        self.setCentralWidget(centralWidget)  

        dialog.setLayout(layout)  
        dialog.exec_()  
  
    def fillList(self):  
        for proc in psutil.process_iter(["pid", "name"]):
            proc_pid = str(proc.info['pid'])
            proc_name = proc.info['name'].lower()
            if "zombie" in proc_name or "plant" in proc_name:
                proc_pid = "#" + proc_pid
            self.listWidget.addItem(
                "进程号：   {:10}，\t进程名:   {}".format(proc_pid, proc_name)
            )
    
    def onRefreshButtonClick(self):  
        self.listWidget.clear()
        self.fillList() 

    def onCommitButtonClick(self):  
        selectedZombie = self.listWidget.currentItem()  
        print(selectedZombie.text())

        # 抽取进程号。这个设计有点问题哈
        processPattern = re.compile(r"进程号：\s*#*([\d]+)\s*，\t进程名:")
        processId = int(processPattern.findall(selectedZombie.text())[0])
        print(processId)
        
        send_msg(task="SET_PROCESS", data={"process_id" : processId})

        msgBox = QMessageBox()  
        msgBox.setIcon(QMessageBox.Information)  
        msgBox.setText("{}\n绑定成功！".format(selectedZombie.text().replace('\t', '').replace(' ', '')))  
        msgBox.setWindowTitle("绑定结果")  
        msgBox.exec_()  
        


    def openAboutDialog(self):  
        dialog = QDialog(self)  
        dialog.setWindowTitle("关于")  
        dialog.setGeometry(200, 200, 300, 200)  
        layout = QVBoxLayout()  
        label = QLabel("作者信息")  
        layout.addWidget(label)  
        dialog.setLayout(layout)  
        dialog.exec_()  