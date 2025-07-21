#pragma once

#include <Dolphin/KVersionControlPlugin>

#include <QMap>

class QLocalSocket;

class LastWatchedPlugin : public KVersionControlPlugin
{
    Q_OBJECT

private:
    enum SendCommandMode {
        WaitForReply,
        SendCommandOnly,
    };

    enum SendCommandTimeout {
        ShortTimeout,
        LongTimeout,
    };

public:
    LastWatchedPlugin(QObject *parent, const QVariantList &args);
    ~LastWatchedPlugin() override;

    QString fileName() const override;

    bool beginRetrieval(const QString &directory) override;
    KVersionControlPlugin::ItemVersion itemVersion(const KFileItem &item) const override;
    void endRetrieval() override;

    QList<QAction *> versionControlActions(const KFileItemList &items) const override;
    QList<QAction *> outOfVersionControlActions(const KFileItemList &items) const override;

private Q_SLOTS:
    void handleContextAction(QAction *action);

private:
    class Private;
    Private *const d;

    static QMap<QString, KVersionControlPlugin::ItemVersion> m_itemVersions;
};
