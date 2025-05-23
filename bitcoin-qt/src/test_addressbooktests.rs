// ---------------- [ File: bitcoin-qt/src/test_addressbooktests.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/test/addressbooktests.h]

#[Q_OBJECT]
pub struct AddressBookTests {
    base: QObject,
    node: Rc<RefCell<dyn NodeInterface>>,
}

impl AddressBookTests {

    pub fn new(node: Rc<RefCell<dyn NodeInterface>>) -> Self {
    
        todo!();
        /*
        : node(node),

        
        */
    }

    #[Q_SLOT]
    pub fn address_book_tests(&mut self)  {
        
        todo!();
        /*
            #ifdef Q_OS_MAC
        if (QApplication::platformName() == "minimal") {
            // Disable for mac on "minimal" platform to avoid crashes inside the Qt
            // framework when it tries to look up unimplemented cocoa functions,
            // and fails to handle returned nulls
            // (https://bugreports.qt.io/browse/QTBUG-49686).
            QWARN("Skipping AddressBookTests on mac build with 'minimal' platform set due to Qt bugs. To run AppTests, invoke "
                  "with 'QT_QPA_PLATFORM=cocoa test_bitcoin-qt' on mac, or else use a linux or windows build.");
            return;
        }
    #endif
        TestAddAddressesToSendBook(m_node);
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/test/addressbooktests.cpp]

/**
  | Fill the edit address dialog box with
  | data, submit it, and ensure that the
  | resulting message meets expectations.
  |
  */
pub fn edit_address_and_submit(
        dialog:       *mut EditAddressDialog,
        label:        &String,
        address:      &String,
        expected_msg: String)  {
    
    todo!();
        /*
            QString warning_text;

        dialog->findChild<QLineEdit*>("labelEdit")->setText(label);
        dialog->findChild<QValidatedLineEdit*>("addressEdit")->setText(address);

        ConfirmMessage(&warning_text, 5);
        dialog->accept();
        QCOMPARE(warning_text, expected_msg);
        */
}

/**
  | Test adding various send addresses
  | to the address book.
  | 
  | There are three cases tested:
  | 
  | - new_address: a new address which should
  | add as a send address successfully.
  | 
  | - existing_s_address: an existing
  | sending address which won't add successfully.
  | 
  | - existing_r_address: an existing
  | receiving address which won't add successfully.
  | 
  | In each case, verify the resulting state
  | of the address book and optionally the
  | warning message presented to the user.
  |
  */
pub fn test_add_addresses_to_send_book(node: Rc<RefCell<dyn NodeInterface>>)  {
    
    todo!();
        /*
            TestChain100Setup test;
        auto wallet_client = typename interfaces::MakeWalletClient(*test.m_node.chain, *Assert(test.m_node.args));
        test.m_node.wallet_client = wallet_client.get();
        node.setContext(&test.m_node);
        std::shared_ptr<CWallet> wallet = std::make_shared<CWallet>(node.context()->chain.get(), "", CreateMockWalletDatabase());
        wallet->LoadWallet();
        wallet->SetWalletFlag(WALLET_FLAG_DESCRIPTORS);
        {
            LOCK(wallet->cs_wallet);
            wallet->SetupDescriptorScriptPubKeyMans();
        }

        auto build_address = [&wallet]() {
            CKey key;
            key.MakeNewKey(true);
            TxDestination dest(GetDestinationForKey(
                key.GetPubKey(), wallet->m_default_address_type));

            return std::make_pair(dest, QString::fromStdString(EncodeDestination(dest)));
        };

        TxDestination r_key_dest, s_key_dest;

        // Add a preexisting "receive" entry in the address book.
        QString preexisting_r_address;
        QString r_label("already here (r)");

        // Add a preexisting "send" entry in the address book.
        QString preexisting_s_address;
        QString s_label("already here (s)");

        // Define a new address (which should add to the address book successfully).
        QString new_address;

        std::tie(r_key_dest, preexisting_r_address) = build_address();
        std::tie(s_key_dest, preexisting_s_address) = build_address();
        std::tie(std::ignore, new_address) = build_address();

        {
            LOCK(wallet->cs_wallet);
            wallet->SetAddressBook(r_key_dest, r_label.toStdString(), "receive");
            wallet->SetAddressBook(s_key_dest, s_label.toStdString(), "send");
        }

        auto check_addbook_size = [&wallet](int expected_size) {
            LOCK(wallet->cs_wallet);
            QCOMPARE(static_cast<int>(wallet->m_address_book.size()), expected_size);
        };

        // We should start with the two addresses we added earlier and nothing else.
        check_addbook_size(2);

        // Initialize relevant QT models.
        std::unique_ptr<const PlatformStyle> platformStyle(PlatformStyle::instantiate("other"));
        OptionsModel optionsModel;
        ClientModel clientModel(node, &optionsModel);
        WalletContext& context = *node.walletClient().context();
        AddWallet(context, wallet);
        WalletModel walletModel(typename interfaces::MakeWallet(context, wallet), clientModel, platformStyle.get());
        RemoveWallet(context, wallet, /* load_on_start= */ std::nullopt);
        EditAddressDialog editAddressDialog(EditAddressDialog::NewSendingAddress);
        editAddressDialog.setModel(walletModel.getAddressTableModel());

        EditAddressAndSubmit(
            &editAddressDialog, QString("uhoh"), preexisting_r_address,
            QString(
                "Address \"%1\" already exists as a receiving address with label "
                "\"%2\" and so cannot be added as a sending address."
                ).arg(preexisting_r_address).arg(r_label));

        check_addbook_size(2);

        EditAddressAndSubmit(
            &editAddressDialog, QString("uhoh, different"), preexisting_s_address,
            QString(
                "The entered address \"%1\" is already in the address book with "
                "label \"%2\"."
                ).arg(preexisting_s_address).arg(s_label));

        check_addbook_size(2);

        // Submit a new address which should add successfully - we expect the
        // warning message to be blank.
        EditAddressAndSubmit(
            &editAddressDialog, QString("new"), new_address, QString(""));

        check_addbook_size(3);
        */
}
