from PyQt5.QtWidgets import QApplication, QMainWindow, QPushButton, QVBoxLayout, QWidget, QTabWidget, QCheckBox, QLabel, QLineEdit, QHBoxLayout  

class showWindow(QMainWindow):
    def __init__(self, main_window) -> None:
        super().__init__() 
        self.main_window = main_window
