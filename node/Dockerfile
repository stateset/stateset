# Copyright 2020, Stateset.

#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at

#    http://www.apache.org/licenses/LICENSE-2.0

#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.

FROM corda/corda-corretto-4.0

LABEL VENDOR="Stateset" \
      MAINTAINER="dom@stateset.io"

COPY --chown=corda:corda . .

# Patch to make compatible
USER root
RUN yum -y install tar && yum clean all
RUN mv corda.jar bin/corda.jar \
 && mv run-corda.sh bin/run-corda \
 && mv cache-nodeInfo.sh bin/cache-nodeInfo \
 && mv stateset-config-generator.sh bin/stateset-config-generator \
 && mv initial-registration.sh bin/initial-registration \
 && ln -s /opt/corda/persistence /opt/corda/artemis \
 && ln -s /opt/corda /opt/stateset
USER corda

# -XX:+UnlockExperimentalVMOptions -XX:+UseCGroupMemoryLimitForHeap -XX:+UseG1GC are added by run-corda.sh
# -Dlog4j2.debug
ENV JVM_ARGS="-XX:+UnlockExperimentalVMOptions -XX:+UnlockDiagnosticVMOptions -XX:+UseCGroupMemoryLimitForHeap -XX:+UseG1GC -XX:NativeMemoryTracking=summary -XX:+PrintNMTStatistics -javaagent:./libs/jolokia-jvm-1.6.0-agent.jar=port=7005,host=localhost"
ENV CORDA_ARGS="--log-to-console --no-local-shell"

CMD ["run-corda"]
# CMD ["sleep","3600"]

