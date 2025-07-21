#include "lastwatchedplugin.h"

#include <KActionCollection>
#include <KFileItem>
#include <KFileItemListProperties>
#include <KLocalizedString>
#include <KPluginFactory>

#include <QDir>
#include <QFileSystemWatcher>
#include <QLocalSocket>
#include <QPointer>
#include <QStringBuilder>

K_PLUGIN_CLASS_WITH_JSON(LastWatchedPlugin, "lastwatchedplugin.json")

class LastWatchedPlugin::Private
{
public:
    Private(LastWatchedPlugin *parent)
        : contextFilePaths()
        , controlSocketPath()
        , controlSocket(new QLocalSocket(parent))
        , databaseFileWatcher(new QFileSystemWatcher(parent))
        , contextActions(new KActionCollection(parent))
    {
    }

    QStringList contextFilePaths;
    QString controlSocketPath;
    QPointer<QLocalSocket> controlSocket;
    QPointer<QLocalSocket> itemStateSocket;
    QPointer<QFileSystemWatcher> databaseFileWatcher;
    QPointer<KActionCollection> contextActions;
};

QMap<QString, KVersionControlPlugin::ItemVersion> LastWatchedPlugin::m_itemVersions;

LastWatchedPlugin::LastWatchedPlugin(QObject *parent, const QVariantList &args)
    : KVersionControlPlugin(parent)
    , d(new Private(this))
{
    Q_UNUSED(args);
}

LastWatchedPlugin::~LastWatchedPlugin()
{
    delete d;
}

QString LastWatchedPlugin::fileName() const
{
    return QStringLiteral(".dropbox");
}

bool LastWatchedPlugin::beginRetrieval(const QString &directory)
{
    return false;
}

KVersionControlPlugin::ItemVersion LastWatchedPlugin::itemVersion(const KFileItem &item) const
{
    return ItemVersion::NormalVersion;
}

void LastWatchedPlugin::endRetrieval()
{
    delete d->itemStateSocket;
}

QList<QAction *> LastWatchedPlugin::versionControlActions(const KFileItemList &items) const
{
    return d->contextActions->actions();
}

QList<QAction *> LastWatchedPlugin::outOfVersionControlActions(const KFileItemList &items) const
{
    Q_UNUSED(items)

    return {};
}

void LastWatchedPlugin::handleContextAction(QAction *action)
{
}

#include "lastwatchedplugin.moc"

#include "moc_lastwatchedplugin.cpp"
