// ---------------- [ File: bitcoin-qt/src/askpassphrasedialog.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/askpassphrasedialog.h]

/**
  | Multifunctional dialog to ask for passphrases.
  | Used for encryption, unlocking, and
  | changing the passphrase.
  |
  */
#[Q_OBJECT]
pub struct AskPassphraseDialog {
    base:           QDialog,
    ui:             *mut UiAskPassphraseDialog,
    mode:           AskPassphraseDialogMode,
    model:          *mut WalletModel,
    caps_lock:      bool,
    passphrase_out: *mut SecureString,
}

pub fn secure_clear_qline_edit(edit: *mut QLineEdit)  {
    
    todo!();
        /*
            // Attempt to overwrite text so that they do not linger around in memory
        edit->setText(QString(" ").repeated(edit->text().size()));
        edit->clear();
        */
}

pub enum AskPassphraseDialogMode {

    /**
      | Ask passphrase twice and encrypt
      |
      */
    Encrypt,    


    /**
      | Ask passphrase and unlock
      |
      */
    Unlock,     


    /**
      | Ask old passphrase + new passphrase
      | twice
      |
      */
    ChangePass, 
}

impl Drop for AskPassphraseDialog {
    fn drop(&mut self) {
        todo!();
        /*
            secureClearPassFields();
        delete ui;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/askpassphrasedialog.cpp]
impl AskPassphraseDialog {
    
    pub fn new(
        mode:           AskPassphraseDialogMode,
        parent:         *mut QWidget,
        passphrase_out: *mut SecureString) -> Self {
    
        todo!();
        /*
            :
        QDialog(parent, gui_util::dialog_flags),
        ui(new UiAskPassphraseDialog),
        mode(_mode),
        model(nullptr),
        fCapsLock(false),
        m_passphrase_out(passphrase_out)
        ui->setupUi(this);

        ui->passEdit1->setMinimumSize(ui->passEdit1->sizeHint());
        ui->passEdit2->setMinimumSize(ui->passEdit2->sizeHint());
        ui->passEdit3->setMinimumSize(ui->passEdit3->sizeHint());

        ui->passEdit1->setMaxLength(MAX_PASSPHRASE_SIZE);
        ui->passEdit2->setMaxLength(MAX_PASSPHRASE_SIZE);
        ui->passEdit3->setMaxLength(MAX_PASSPHRASE_SIZE);

        // Setup Caps Lock detection.
        ui->passEdit1->installEventFilter(this);
        ui->passEdit2->installEventFilter(this);
        ui->passEdit3->installEventFilter(this);

        switch(mode)
        {
            case Encrypt: // Ask passphrase x2
                ui->warningLabel->setText(tr("Enter the new passphrase for the wallet.<br/>Please use a passphrase of <b>ten or more random characters</b>, or <b>eight or more words</b>."));
                ui->passLabel1->hide();
                ui->passEdit1->hide();
                setWindowTitle(tr("Encrypt wallet"));
                break;
            case Unlock: // Ask passphrase
                ui->warningLabel->setText(tr("This operation needs your wallet passphrase to unlock the wallet."));
                ui->passLabel2->hide();
                ui->passEdit2->hide();
                ui->passLabel3->hide();
                ui->passEdit3->hide();
                setWindowTitle(tr("Unlock wallet"));
                break;
            case ChangePass: // Ask old passphrase + new passphrase x2
                setWindowTitle(tr("Change passphrase"));
                ui->warningLabel->setText(tr("Enter the old passphrase and new passphrase for the wallet."));
                break;
        }
        textChanged();
        connect(ui->toggleShowPasswordButton, &QPushButton::toggled, this, &AskPassphraseDialog::toggleShowPassword);
        connect(ui->passEdit1, &QLineEdit::textChanged, this, &AskPassphraseDialog::textChanged);
        connect(ui->passEdit2, &QLineEdit::textChanged, this, &AskPassphraseDialog::textChanged);
        connect(ui->passEdit3, &QLineEdit::textChanged, this, &AskPassphraseDialog::textChanged);

        gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn set_model(&mut self, model: *mut WalletModel)  {
        
        todo!();
        /*
            this->model = _model;
        */
    }
    
    pub fn accept(&mut self)  {
        
        todo!();
        /*
            SecureString oldpass, newpass1, newpass2;
        if (!model && mode != Encrypt)
            return;
        oldpass.reserve(MAX_PASSPHRASE_SIZE);
        newpass1.reserve(MAX_PASSPHRASE_SIZE);
        newpass2.reserve(MAX_PASSPHRASE_SIZE);
        // TODO: get rid of this .c_str() by implementing SecureString::operator=(std::string)
        // Alternately, find a way to make this input mlock()'d to begin with.
        oldpass.assign(ui->passEdit1->text().toStdString().c_str());
        newpass1.assign(ui->passEdit2->text().toStdString().c_str());
        newpass2.assign(ui->passEdit3->text().toStdString().c_str());

        secureClearPassFields();

        switch(mode)
        {
        case Encrypt: {
            if(newpass1.empty() || newpass2.empty())
            {
                // Cannot encrypt with empty passphrase
                break;
            }
            QMessageBox::StandardButton retval = QMessageBox::question(this, tr("Confirm wallet encryption"),
                     tr("Warning: If you encrypt your wallet and lose your passphrase, you will <b>LOSE ALL OF YOUR BITCOINS</b>!") + "<br><br>" + tr("Are you sure you wish to encrypt your wallet?"),
                     QMessageBox::Yes|QMessageBox::Cancel,
                     QMessageBox::Cancel);
            if(retval == QMessageBox::Yes)
            {
                if(newpass1 == newpass2)
                {
                    QString encryption_reminder = tr("Remember that encrypting your wallet cannot fully protect "
                    "your bitcoins from being stolen by malware infecting your computer.");
                    if (m_passphrase_out) {
                        m_passphrase_out->assign(newpass1);
                        QMessageBox::warning(this, tr("Wallet to be encrypted"),
                                             "<qt>" +
                                             tr("Your wallet is about to be encrypted. ") + encryption_reminder +
                                             "</b></qt>");
                    } else {
                        assert(model != nullptr);
                        if (model->setWalletEncrypted(newpass1)) {
                            QMessageBox::warning(this, tr("Wallet encrypted"),
                                                 "<qt>" +
                                                 tr("Your wallet is now encrypted. ") + encryption_reminder +
                                                 "<br><br><b>" +
                                                 tr("IMPORTANT: Any previous backups you have made of your wallet file "
                                                 "should be replaced with the newly generated, encrypted wallet file. "
                                                 "For security reasons, previous backups of the unencrypted wallet file "
                                                 "will become useless as soon as you start using the new, encrypted wallet.") +
                                                 "</b></qt>");
                        } else {
                            QMessageBox::critical(this, tr("Wallet encryption failed"),
                                                 tr("Wallet encryption failed due to an internal error. Your wallet was not encrypted."));
                        }
                    }
                    QDialog::accept(); // Success
                }
                else
                {
                    QMessageBox::critical(this, tr("Wallet encryption failed"),
                                         tr("The supplied passphrases do not match."));
                }
            }
            else
            {
                QDialog::reject(); // Cancelled
            }
            } break;
        case Unlock:
            try {
                if (!model->setWalletLocked(false, oldpass)) {
                    QMessageBox::critical(this, tr("Wallet unlock failed"),
                                          tr("The passphrase entered for the wallet decryption was incorrect."));
                } else {
                    QDialog::accept(); // Success
                }
            } catch (const std::runtime_error& e) {
                QMessageBox::critical(this, tr("Wallet unlock failed"), e.what());
            }
            break;
        case ChangePass:
            if(newpass1 == newpass2)
            {
                if(model->changePassphrase(oldpass, newpass1))
                {
                    QMessageBox::information(this, tr("Wallet encrypted"),
                                         tr("Wallet passphrase was successfully changed."));
                    QDialog::accept(); // Success
                }
                else
                {
                    QMessageBox::critical(this, tr("Wallet encryption failed"),
                                         tr("The passphrase entered for the wallet decryption was incorrect."));
                }
            }
            else
            {
                QMessageBox::critical(this, tr("Wallet encryption failed"),
                                     tr("The supplied passphrases do not match."));
            }
            break;
        }
        */
    }
    
    #[Q_SLOT]
    pub fn text_changed(&mut self)  {
        
        todo!();
        /*
            // Validate input, set Ok button to enabled when acceptable
        bool acceptable = false;
        switch(mode)
        {
        case Encrypt: // New passphrase x2
            acceptable = !ui->passEdit2->text().isEmpty() && !ui->passEdit3->text().isEmpty();
            break;
        case Unlock: // Old passphrase x1
            acceptable = !ui->passEdit1->text().isEmpty();
            break;
        case ChangePass: // Old passphrase x1, new passphrase x2
            acceptable = !ui->passEdit1->text().isEmpty() && !ui->passEdit2->text().isEmpty() && !ui->passEdit3->text().isEmpty();
            break;
        }
        ui->buttonBox->button(QDialogButtonBox::Ok)->setEnabled(acceptable);
        */
    }
    
    pub fn event(&mut self, event: *mut QEvent) -> bool {
        
        todo!();
        /*
            // Detect Caps Lock key press.
        if (event->type() == QEvent::KeyPress) {
            QKeyEvent *ke = static_cast<QKeyEvent *>(event);
            if (ke->key() == QtKey_CapsLock) {
                fCapsLock = !fCapsLock;
            }
            if (fCapsLock) {
                ui->capsLabel->setText(tr("Warning: The Caps Lock key is on!"));
            } else {
                ui->capsLabel->clear();
            }
        }
        return QWidget::event(event);
        */
    }
    
    #[Q_SLOT]
    pub fn toggle_show_password(&mut self, show: bool)  {
        
        todo!();
        /*
            ui->toggleShowPasswordButton->setDown(show);
        const auto mode = show ? QLineEdit::Normal : QLineEdit::Password;
        ui->passEdit1->setEchoMode(mode);
        ui->passEdit2->setEchoMode(mode);
        ui->passEdit3->setEchoMode(mode);
        */
    }
    
    pub fn event_filter(&mut self, 
        object: *mut QObject,
        event:  *mut QEvent) -> bool {
        
        todo!();
        /*
            /* Detect Caps Lock.
         * There is no good OS-independent way to check a key state in Qt, but we
         * can detect Caps Lock by checking for the following condition:
         * Shift key is down and the result is a lower case character, or
         * Shift key is not down and the result is an upper case character.
         */
        if (event->type() == QEvent::KeyPress) {
            QKeyEvent *ke = static_cast<QKeyEvent *>(event);
            QString str = ke->text();
            if (str.length() != 0) {
                const QChar *psz = str.unicode();
                bool fShift = (ke->modifiers() & QtShiftModifier) != 0;
                if ((fShift && *psz >= 'a' && *psz <= 'z') || (!fShift && *psz >= 'A' && *psz <= 'Z')) {
                    fCapsLock = true;
                    ui->capsLabel->setText(tr("Warning: The Caps Lock key is on!"));
                } else if (psz->isLetter()) {
                    fCapsLock = false;
                    ui->capsLabel->clear();
                }
            }
        }
        return QDialog::eventFilter(object, event);
        */
    }
    
    #[Q_SLOT]
    pub fn secure_clear_pass_fields(&mut self)  {
        
        todo!();
        /*
            SecureClearQLineEdit(ui->passEdit1);
        SecureClearQLineEdit(ui->passEdit2);
        SecureClearQLineEdit(ui->passEdit3);
        */
    }
}
