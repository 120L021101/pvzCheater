import sys  
from PyQt5.QtWidgets import QComboBox, QPushButton, QAction, \
                            QDialog, QApplication, QMainWindow, \
                            QMenuBar, QVBoxLayout, QWidget, \
                            QTabWidget, QCheckBox, QLabel, \
                            QLineEdit, QHBoxLayout, QListWidget \

import random

from zombie.layout import zombieWindow
from plant.layout import plantWindow
from ofunc.layout import miscWindow
from menu.layout import MenuWindow

class PlantVsZombieModifier(QMainWindow):  
    def __init__(self):  
        super().__init__()  
  
        self.setWindowTitle("植物大战僵尸修改器")  
        self.setGeometry(100, 100, 600, 400)  
  
        # 创建基本布局 
        self.tab_widget = QTabWidget()  
        self.menuBar = QMenuBar(self)  
        self.setMenuBar(self.menuBar)

        # 创建菜单栏
        self.menu_window = MenuWindow(self.menuBar)

        # 创建植物选项卡
        self.plant_window = plantWindow(self.tab_widget)

        # 创建僵尸选项卡
        self.zombie_window = zombieWindow(self.tab_widget)
  
        # 创建杂项选项卡
        self.zombie_window = miscWindow(self.tab_widget)

        # 将tab_widget设置为窗口的中心组件  
        self.setCentralWidget(self.tab_widget)  


if __name__ == '__main__':  
    app = QApplication(sys.argv)  
    main_window = PlantVsZombieModifier()  
    main_window.show()  
    sys.exit(app.exec_())